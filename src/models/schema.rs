table! {
    completed (id) {
        id -> Int4,
        team_id -> Nullable<Int4>,
        task_id -> Nullable<Int4>,
        time -> Timestamp,
    }
}

table! {
    game (id) {
        id -> Int4,
        start_game -> Timestamp,
        end_game -> Timestamp,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        task_name -> Varchar,
        description -> Varchar,
        flag -> Varchar,
        points -> Varchar,
        keys_reward -> Json,
        keys_condition -> Json,
        coords -> Json,
    }
}

table! {
    team_game (id) {
        id -> Int4,
        team_id -> Nullable<Int4>,
        keys_owned -> Json,
        points -> Int4,
    }
}

table! {
    team_info (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        country -> Nullable<Varchar>,
        university -> Nullable<Varchar>,
        token -> Varchar,
    }
}

joinable!(completed -> tasks (task_id));
joinable!(completed -> team_info (team_id));
joinable!(team_game -> team_info (team_id));

allow_tables_to_appear_in_same_query!(
    completed,
    game,
    tasks,
    team_game,
    team_info,
);
