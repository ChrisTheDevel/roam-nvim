//! this module contains the schema definitions. These make compile time validations interactions
//! with the sqlite database possible.

table! {
    Aliases (alias) {
        alias -> Text,
        filepath -> Text,
    }
}

table! {
    Files (filepath) {
        filepath -> Text,
        title -> Text,
        atime -> Integer,
        mtime -> Integer,
    }
}

table! {
    Links (id) {
        id -> Integer,
        source -> Text,
        destination -> Text,
        linktype -> Text,
    }
}

joinable!(Aliases -> Files (filepath));

allow_tables_to_appear_in_same_query!(
    Aliases,
    Files,
    Links,
);
