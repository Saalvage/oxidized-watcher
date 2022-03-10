CREATE TABLE notes (
    id BIGSERIAL PRIMARY KEY NOT NULL,
    kind SMALLINT NOT NULL CHECK (kind > 0 AND kind <= 4), -- The kind of the note (e.g. warning, ban)
    user_id BIGINT NOT NULL, -- The user the note was issued for
    issuer_id BIGINT NOT NULL, -- The user who issued the note
    info TEXT NOT NULL -- Additional custom text
)
