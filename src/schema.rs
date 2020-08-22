table! {
    permissions (id) {
        id -> Varchar,
        name -> Nullable<Varchar>,
    }
}

table! {
    roles (id) {
        id -> Varchar,
        name -> Nullable<Varchar>,
    }
}

table! {
    roles_permissions (id) {
        id -> Varchar,
        role_id -> Nullable<Varchar>,
        permission_id -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        pw_hash -> Varchar,
    }
}

table! {
    users_roles (id) {
        id -> Varchar,
        user_id -> Nullable<Varchar>,
        role_id -> Nullable<Varchar>,
    }
}

joinable!(roles_permissions -> permissions (permission_id));
joinable!(roles_permissions -> roles (role_id));
joinable!(users_roles -> roles (role_id));
joinable!(users_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    permissions,
    roles,
    roles_permissions,
    users,
    users_roles,
);
