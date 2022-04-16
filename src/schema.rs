table! {
    Aliases (alias) {
        alias -> Nullable<Text>,
        filepath -> Nullable<Text>,
    }
}

table! {
    Files (filepath) {
        filepath -> Nullable<Text>,
        title -> Nullable<Text>,
        atime -> Nullable<Integer>,
        mtime -> Nullable<Integer>,
    }
}

table! {
    Links (id) {
        id -> Integer,
        source -> Nullable<Text>,
        destination -> Nullable<Text>,
        linktype -> Nullable<Text>,
    }
}

joinable!(Aliases -> Files (filepath));

allow_tables_to_appear_in_same_query!(
    Aliases,
    Files,
    Links,
);
