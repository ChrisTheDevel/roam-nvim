table! {
    Aliases (alias) {
        alias -> Nullable<Text>,
        filepath -> Text,
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

table! {
    Nodes (filepath) {
        filepath -> Text,
        title -> Text,
        atime -> Integer,
        mtime -> Integer,
    }
}

joinable!(Aliases -> Nodes (filepath));

allow_tables_to_appear_in_same_query!(
    Aliases,
    Links,
    Nodes,
);
