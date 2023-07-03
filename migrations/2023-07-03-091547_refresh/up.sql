-- This table keeps track of all the individual refresh token chains.
CREATE TABLE refresh_chain (
   id INTEGER NOT NULL PRIMARY KEY,

   -- A chain belongs to a single user.
   user INTEGER NOT NULL REFERENCES users(id),

   -- If a chain is revoked, any corresponding tokens are invalid.
   revoked_at INTEGER,

   -- Metadata.
   created_at INTEGER NOT NULL DEFAULT (UNIXEPOCH()),
   updated_at INTEGER
);

-- An individual token, granting access to a authentication token. We must
-- store these in the database to make sure they're not replayed.
CREATE TABLE refresh_token (
    id INTEGER NOT NULL PRIMARY KEY,

    -- A token can only be used once, and up until a given time.
    used_at INTEGER,

    -- A token belongs to a chain.
    refresh_chain INTEGER NOT NULL REFERENCES refresh_chain(id),

    -- Metadata.
    created_at INTEGER NOT NULL DEFAULT (UNIXEPOCH()),
    updated_at INTEGER
);
