table! {
    notes (id) {
        id -> Integer,
        kind -> SmallInt,
        user -> BigInt,
        issuer -> BigInt,
        info -> Text,
    }
}
