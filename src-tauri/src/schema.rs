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
        source -> Text,
        structure -> Text,
        mol -> Text,
        molfomula -> Text,
        comno -> Text,
        info -> Text,
        new -> Text,
        oneh -> Text,
        cc -> Text,
        hsqc -> Text,
        hmbc -> Text,
        cosy -> Text,
        hrms -> Text,
        ir -> Text,
        uv -> Text,
        xray -> Text,
        note -> Text,
        charger -> Text,
        sdate -> Text,
    }
}

diesel::table! {
    ele (id) {
        id -> Integer,
        number -> Text,
        name -> Text,
        catlog -> Text,
        class -> Text,
        source -> Text,
        describe-> Text,
        detail-> Text,
        size-> Text,
        regno-> Text,
        researcher-> Text,
        seqinfo-> Text,
        sdate -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    ba,
    com,
    ele,
);
