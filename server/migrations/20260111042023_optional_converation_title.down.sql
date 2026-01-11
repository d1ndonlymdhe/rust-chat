-- Add down migration script here
ALTER TABLE conversation
    ALTER COLUMN title SET NOT NULL;