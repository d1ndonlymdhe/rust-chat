use macros::{db_err, db_func};
use shared::db::signup::IdOnly;
use sqlx::{query, query_as};
use shared::AnyErr;

#[db_err]
enum CreateConversationError {
    AlreadyExists,
}


#[db_func]
async fn create_conversation(name: &str, member_user_ids: Vec<i32>)-> Result<String, CreateConversationError> {
    let mut txn = pool.begin().await.unwrap();


    let check_conversation_exists = query_as!(IdOnly,
        "SELECT cm.conversation_id as id FROM conversation_member cm
        GROUP BY cm.conversation_id
        HAVING COUNT(cm.user_id) = $1 AND
        BOOL_AND(cm.user_id = ANY($2))
        ", member_user_ids.len() as i64, member_user_ids.as_slice())
        .fetch_optional(&mut *txn)
        .await
        ?;


    if check_conversation_exists.is_none() {
        let create_conversation = query_as!(IdOnly,"INSERT INTO conversation (title) VALUES ($1) RETURNING id", name)
            .fetch_one(&mut *txn)
            .await?;

        let conversation_id = create_conversation.id;

        for user_id in member_user_ids {
            query!("INSERT INTO conversation_member (conversation_id, user_id) VALUES ($1, $2)", conversation_id, user_id)
                .execute(&mut *txn)
                .await?;
        }
        txn.commit().await.unwrap();
        return Ok(conversation_id.to_string());
    }else{
        return Err(CreateConversationError::AlreadyExists);
    };

}