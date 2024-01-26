// @generated automatically by Diesel CLI.

diesel::table! {
    ba (id) {
        id -> Integer,
        number -> Text,
        code -> Text,
        nameen -> Text,
        namecn -> Text,
        source -> Text,
        place -> Text,
        org -> Text,
        research -> Text,
        sdate -> Text,
    }
}

diesel::table! {
    com (id) {
        id -> Integer,
        number -> Text,
        code -> Text,
        nameen -> Text,
        namecn -> Text,
        source -> Text,
        place -> Text,
        org -> Text,
        research -> Text,
        sdate -> Text,
    }
}

diesel::table! {
    element (id) {
        id -> Integer,
        number -> Text,
        code -> Text,
        nameen -> Text,
        namecn -> Text,
        source -> Text,
        place -> Text,
        org -> Text,
        research -> Text,
        sdate -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    ba,
    com,
    element,
);
