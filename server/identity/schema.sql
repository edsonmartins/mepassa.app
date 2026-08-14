-- ZapLivre Identity Server Database Schema
-- PostgreSQL 15+
--
-- NOTE: the canonical schema lives in server/postgres/init.sql (loaded by
-- docker-compose). This file mirrors the `usernames` definition there for
-- standalone deployments of the identity server. Keep both in sync.

-- Usernames table: maps @username to Peer ID + Prekey Bundle (ADR 001)
CREATE TABLE IF NOT EXISTS usernames (
    -- Username (unique, 3-20 chars, lowercase alphanumeric + underscore)
    username TEXT PRIMARY KEY,

    -- Peer ID mapping (one username per peer)
    peer_id TEXT NOT NULL UNIQUE,

    -- Public key (for verification)
    public_key BYTEA NOT NULL,

    -- Prekey bundle for X3DH (stored as JSONB)
    prekey_bundle JSONB NOT NULL,

    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    -- Username format validation (3-20 chars, lowercase alphanumeric + underscore)
    CONSTRAINT username_format CHECK (username ~ '^[a-z0-9_]{3,20}$')
);

-- Indexes for fast lookups
CREATE INDEX IF NOT EXISTS idx_usernames_peer_id ON usernames(peer_id);
CREATE INDEX IF NOT EXISTS idx_usernames_created_at ON usernames(created_at DESC);

-- Append-only identity log. Each entry commits to the previous entry hash.
CREATE TABLE IF NOT EXISTS key_transparency_log (
    sequence BIGSERIAL PRIMARY KEY,
    peer_id TEXT NOT NULL,
    public_key BYTEA NOT NULL,
    previous_hash BYTEA NOT NULL,
    entry_hash BYTEA NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_key_transparency_peer ON key_transparency_log(peer_id, sequence DESC);
