table! {
    deployments (id) {
        id -> Uuid,
        project_id -> Uuid,
        version -> Varchar,
        hash -> Bpchar,
        has_static -> Bool,
        published_at -> Timestamp,
    }
}

table! {
    handlers (id) {
        id -> Uuid,
        deployment_id -> Uuid,
        name -> Text,
        query_parameters -> Nullable<Array<Text>>,
        headers -> Nullable<Array<Text>>,
        path_parameters -> Nullable<Array<Text>>,
        body -> Nullable<Jsonb>,
        logic -> Jsonb,
    }
}

table! {
    projects (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    routes (id) {
        id -> Uuid,
        deployment_id -> Uuid,
        path -> Text,
        methods -> Array<Text>,
        handler -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        username -> Varchar,
        password -> Text,
        created_at -> Timestamp,
    }
}

joinable!(deployments -> projects (project_id));
joinable!(handlers -> deployments (deployment_id));
joinable!(projects -> users (user_id));
joinable!(routes -> deployments (deployment_id));

allow_tables_to_appear_in_same_query!(
    deployments,
    handlers,
    projects,
    routes,
    users,
);
