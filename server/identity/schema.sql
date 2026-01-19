-- MePassa Identity Server Database Schema
-- PostgreSQL 15+

-- Usernames table: maps @username to Peer ID + Prekey Bundle
CREATE TABLE IF NOT EXISTS usernames (
    id SERIAL PRIMARY KEY,
    username VARCHAR(20) UNIQUE NOT NULL,
    peer_id TEXT NOT NULL,
    public_key BYTEA NOT NULL,
    prekey_bundle JSONB NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_updated TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Indexes for fast lookups
CREATE INDEX IF NOT EXISTS idx_username ON usernames(username);
CREATE INDEX IF NOT EXISTS idx_peer_id ON usernames(peer_id);

-- Trigger to auto-update last_updated
CREATE OR REPLACE FUNCTION update_last_updated()
RETURNS TRIGGER AS $$
BEGIN
    NEW.last_updated = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_last_updated
BEFORE UPDATE ON usernames
FOR EACH ROW
EXECUTE FUNCTION update_last_updated();
