-- Add down migration script here
ALTER TABLE message
    DROP CONSTRAINT message_sender_user_id_fkey;

ALTER TABLE message
    DROP COLUMN sender_user_id;

ALTER TABLE message
    ADD COLUMN sender_member_id INTEGER NOT NULL;

ALTER TABLE message
    ADD CONSTRAINT message_sender_member_id_fkey
    FOREIGN KEY (sender_member_id) REFERENCES conversation_member(id) ON DELETE CASCADE;