CREATE TABLE notes (
    id INTEGER PRIMARY KEY NOT NULL,
    kind SMALLINT NOT NULL, -- The kind of the note (e.g. warning, ban)
    user BIGINT NOT NULL, -- The user the note was issued for
    issuer BIGINT NOT NULL, -- The person who issued the note
    info TEXT NOT NULL -- Additional custom text
)
