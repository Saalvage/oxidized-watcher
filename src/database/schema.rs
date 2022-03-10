table! {
    notes (id) {
        id -> Int8,
        kind -> Int2,
        user_id -> Int8,
        issuer_id -> Int8,
        info -> Text,
    }
}
