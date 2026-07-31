CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nick TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

ALTER TABLE battles ADD COLUMN created_by TEXT NOT NULL REFERENCES users(nick);
CREATE INDEX idx_battles_created_by ON battles (created_by);
