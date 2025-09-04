use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default)]
pub struct ImportTask {
    title: TaskTitle,
    author: String,
    tags: String,

    character: i32,
    place: i32,

    flag: Flag,

    keys_meta: KeysMeta,  // Is it needed? IDK

    points: u32,  // Changeable
}

#[derive(Default)]
struct TaskTitle {
    ru: String,
    en: Option<String>,
}

#[derive(Deserialize, Default)]
struct Flag {
    value: String,
    is_flag_regexp: bool,
}

#[derive(Deserialize, Default)]
struct KeysMeta {
    keys_reward: Vec<Vec<i32>>,
    keys_condition: Vec<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Map {
    pub places: Vec<Place>,
    pub keys: Vec<Key>,
    pub characters: Vec<Character>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Place {
    pub id: i32,
    pub name: String,
    pub coords: Vec<i32>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Key {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Character {
    pub id: i32,
    pub name: String,
}