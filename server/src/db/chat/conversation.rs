use macros::{db_err, db_func};
use shared::{db::signup::{IdOnly, User}, routes::chat::conversation::{ConversationMember, CreateConversationResponse}};
use sqlx::{query, query_as};
use shared::AnyErr;

#[db_err]
pub enum CreateConversationError {
    InvalidUsers,
}

struct ConversationIdAndName {
    id: i32,
    title: Option<String>,
}

#[db_func]
pub async fn create_conversation(name: Option<String>, member_user_ids: Vec<i32>)-> Result<CreateConversationResponse, CreateConversationError> {
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
        let create_conversation = query_as!(IdOnly,"INSERT INTO conversation (title,conv_type) VALUES ($1, 'group') RETURNING id", name)
            .fetch_one(&mut *txn)
            .await?;

        let conversation_id = create_conversation.id;

        // TODO: Optimize this with bulk insert
        for user_id in member_user_ids.iter() {
            query!("INSERT INTO conversation_member (conversation_id, user_id, role) VALUES ($1, $2, 'member')", conversation_id, user_id)
                .execute(&mut *txn)
                .await?;
        }

        let users_in_conversation = sqlx::query_as!(ConversationMember,r#"SELECT id as user_id,username from users where id = ANY($1)"#, &member_user_ids).fetch_all(&mut *txn).await?;
        if users_in_conversation.len() != member_user_ids.len() {
            return Err(CreateConversationError::InvalidUsers);
        }

        txn.commit().await.unwrap();
        
        return Ok(CreateConversationResponse { conversation_id: conversation_id.to_string(), title: None, members: users_in_conversation });
    }else{
        let check_conversation_exists = check_conversation_exists.unwrap();
        let users_in_conversation = sqlx::query_as!(ConversationMember,r#"SELECT id as user_id,username from users where id = ANY($1)"#, &member_user_ids).fetch_all(&mut *txn).await?;
        if users_in_conversation.len() != member_user_ids.len() {
            return Err(CreateConversationError::InvalidUsers);
        }

        return Ok(
            CreateConversationResponse { conversation_id: check_conversation_exists.id.to_string(), title: check_conversation_exists.title, members: users_in_conversation }
        )

    };

}