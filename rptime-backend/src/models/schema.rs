table! {
    completed (id) {
        id -> Int4,
        team_id -> Int4,
        task_id -> Int4,
        flag -> Varchar,
        solved -> Bool,
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
        title_ru -> Varchar,
        title_en -> Nullable<Varchar>,
        description_ru -> Varchar,
        description_en -> Nullable<Varchar>,
        flag -> Varchar,
        points -> Int4,
        keys_reward -> Array<Int4>,
        keys_condition -> Array<Int4>,
        place -> Int4,
        author -> Varchar,
        character -> Int4,
        tags -> Varchar,
    }
}

table! {
    team_game (id) {
        id -> Int4,
        team_id -> Int4,
        keys_owned -> Array<Int4>,
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

allow_tables_to_appear_in_same_query!(completed, game, tasks, team_game, team_info,);
