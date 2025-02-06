diesel::table! {
    dashboards (id) {
        id -> Integer,
        title -> Text,
        subtitle -> Text,
        description -> Text,
        project_id -> Integer,
    }
}

diesel::table! {
    components (id) {
        id -> Integer,
        dashboard_id -> Nullable<Integer>,
        parent_id -> Nullable<Integer>,
        #[sql_name = "type"]
        comp_type -> Text,
    }
}

diesel::table! {
    variables (id) {
        id -> Integer,
        ref_id -> Nullable<VarChar>,
        /// Default value for the variable (optional) in the form { type: "string", value: "hi" }
        default -> Nullable<Text>,
        value -> Text,
        dashboard_id -> Integer,
        project_id -> Integer,
    }
}

diesel::table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        path -> Text,
    }
}

diesel::table! {
    bindings (id) {
        id -> Integer,
        project_id -> Integer,
        producer_ref -> Text,
        producer_type -> Text,
        consumer_ref -> Text,
        consumer_type -> Text,
    }
}

diesel::joinable!(variables -> projects (project_id));
diesel::joinable!(variables -> dashboards (dashboard_id));
diesel::joinable!(components -> dashboards (dashboard_id));
diesel::joinable!(dashboards -> projects (project_id));
diesel::joinable!(bindings -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(projects, variables);
diesel::allow_tables_to_appear_in_same_query!(projects, dashboards);
diesel::allow_tables_to_appear_in_same_query!(projects, bindings);
diesel::allow_tables_to_appear_in_same_query!(dashboards, variables);
diesel::allow_tables_to_appear_in_same_query!(dashboards, components);
