-- Add up migration script here
CREATE TABLE IF NOT EXISTS conversation (
    id SERIAL NOT NULL PRIMARY KEY,
    conv_type TEXT NOT NULL,
    title TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS conversation_member (
    id SERIAL NOT NULL PRIMARY KEY,
    conversation_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    role TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (conversation_id) REFERENCES conversation(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS text_message_content (
    id SERIAL NOT NULL PRIMARY KEY,
    text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS message (
    id SERIAL NOT NULL PRIMARY KEY,
    conversation_id INTEGER NOT NULL,
    sender_member_id INTEGER NOT NULL,
    message_type TEXT NOT NULL,
    message_content_id INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (conversation_id) REFERENCES conversation(id) ON DELETE CASCADE,
    FOREIGN KEY (sender_member_id) REFERENCES conversation_member(id) ON DELETE CASCADE,
    FOREIGN KEY (message_content_id) REFERENCES text_message_content(id) ON DELETE CASCADE
);

