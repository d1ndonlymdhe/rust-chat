-- Add up migration script here
ALTER TABLE conversation
    ALTER COLUMN title DROP NOT NULL;