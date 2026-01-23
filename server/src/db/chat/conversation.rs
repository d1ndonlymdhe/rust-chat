use macros::{db_err, db_func};
use shared::AnyErr;
use shared::{
    db::signup::{IdOnly},
    routes::chat::conversation::{ConversationMember, CreateConversationResponse, ConversationWithMembers},
};
use sqlx::types::Json;
use sqlx::{query, query_as};

#[db_err]
pub enum CreateConversationError {
    InvalidUsers,
}

struct ConversationIdAndName {
    id: i32,
    title: Option<String>,
}

#[db_func]
pub async fn create_conversation(
    name: Option<String>,
    member_user_ids: Vec<i32>,
) -> Result<CreateConversationResponse, CreateConversationError> {
    let mut txn = pool.begin().await.unwrap();

    let check_conversation_exists = query_as!(ConversationIdAndName,
        "SELECT cm.conversation_id as id, c.title as title FROM conversation_member cm JOIN conversation c on c.id = cm.conversation_id
        GROUP BY cm.conversation_id, c.title
        HAVING COUNT(cm.user_id) = $1
        AND COUNT(*) FILTER (WHERE cm.user_id = ANY($2)) = $1
        ", member_user_ids.len() as i64, member_user_ids.as_slice())
        .fetch_optional(&mut *txn)
        .await
        ?;

    if check_conversation_exists.is_none() {
        let create_conversation = query_as!(
            IdOnly,
            "INSERT INTO conversation (title,conv_type) VALUES ($1, 'group') RETURNING id",
            name
        )
        .fetch_one(&mut *txn)
        .await?;

        let conversation_id = create_conversation.id;

        // TODO: Optimize this with bulk insert
        for user_id in member_user_ids.iter() {
            query!("INSERT INTO conversation_member (conversation_id, user_id, role) VALUES ($1, $2, 'member')", conversation_id, user_id)
                .execute(&mut *txn)
                .await?;
        }

        let users_in_conversation = sqlx::query_as!(
            ConversationMember,
            r#"SELECT id as user_id,username from users where id = ANY($1)"#,
            &member_user_ids
        )
        .fetch_all(&mut *txn)
        .await?;
        if users_in_conversation.len() != member_user_ids.len() {
            return Err(CreateConversationError::InvalidUsers);
        }

        txn.commit().await.unwrap();

        return Ok(CreateConversationResponse {
            conversation_id: conversation_id.to_string(),
            title: None,
            members: users_in_conversation,
        });
    } else {
        let check_conversation_exists = check_conversation_exists.unwrap();
        let users_in_conversation = sqlx::query_as!(
            ConversationMember,
            r#"SELECT id as user_id,username from users where id = ANY($1)"#,
            &member_user_ids
        )
        .fetch_all(&mut *txn)
        .await?;
        if users_in_conversation.len() != member_user_ids.len() {
            return Err(CreateConversationError::InvalidUsers);
        }

        return Ok(CreateConversationResponse {
            conversation_id: check_conversation_exists.id.to_string(),
            title: check_conversation_exists.title,
            members: users_in_conversation,
        });
    };
}



#[db_func]
pub async fn get_user_conversations(user_id: i32) -> Result<Vec<ConversationWithMembers>, sqlx::Error> {
    let conversations = query_as!(ConversationWithMembers,r#"
        with conversation_user as(
            select 
            conversation_id,
            u."id" as user_id, 
            username from 
            users u 
            inner join conversation_member cm 
            on cm.user_id = u.id)
        select 
        c.*, 
        coalesce(
            jsonb_agg(
                jsonb_build_object(
                    'user_id' ,cu.user_id,
                    'username', cu.username
                )
            ),
            '[]'::jsonb
        ) as "members!: Json<Vec<ConversationMember>>" 
        from 
        conversation_user cu 
        inner join conversation c 
        on c.id = cu.conversation_id 
        where 
        cu.conversation_id in 
            (select 
            cm.conversation_id 
            from
            conversation_member cm
            where 
            cm.user_id = $1)
        group by
        c.id
        "#, user_id).fetch_all(pool).await?;
    return Ok(conversations);
}
