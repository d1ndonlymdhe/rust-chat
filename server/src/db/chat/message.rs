use macros::db_func;
use shared::{db::signup::IdOnly, routes::chat::message::{Message, RawDbMessage}};
use sqlx::query_as;

#[db_func]
pub async fn get_messages_for_conversation(
    conversation_id: i32,
    user_id: i32,
)-> Result<Vec<Message>, sqlx::Error> {
    let res = query_as!(
        Message,
        "
        SELECT
            m.id,
            m.conversation_id,
            m.sender_member_id,
            m.message_type,
            m.message_content_id,
            tmc.text as content,
            m.created_at,
            m.updated_at 
        FROM message m
        INNER JOIN conversation c
        ON m.conversation_id = c.id
        INNER JOIN text_message_content tmc
        on m.message_content_id = tmc.id
        INNER JOIN conversation_member cm
        ON c.id = cm.conversation_id
        WHERE cm.user_id = $1 AND
        c.id = $2
        ",
        user_id, conversation_id
    ).fetch_all(pool).await?;
    Ok(res)
}

#[db_func]
pub async fn add_text_message(
    conversation_id: i32,
    sender_member_id: i32,
    text: &str,
) -> Result<Message, sqlx::Error> {
    let mut txn = pool.begin().await?;
    let tmc_id = query_as!(IdOnly,"
    INSERT INTO
    text_message_content (text)
    values ($1)
    RETURNING id
    ",text).fetch_one(&mut *txn).await?;

    let msg = query_as!(RawDbMessage,"
    INSERT INTO
    message (conversation_id, sender_member_id, message_type, message_content_id)
    values ($1, $2, 'text', $3)
    RETURNING
        id,
        conversation_id,
        sender_member_id,
        message_type,
        message_content_id,
        created_at,
        updated_at
    ", conversation_id, sender_member_id, tmc_id.id).fetch_one(&mut *txn).await?;
    
    Ok(Message{
        content: text.to_string(),
        ..msg.into()
    })
}