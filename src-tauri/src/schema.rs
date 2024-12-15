diesel::table! {
    dashboards (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
    }
}
