use crate::models::InsertTask;
use failure::Fallible;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};
use toml::from_str;

#[derive(Deserialize, Default)]
struct ImportTask {
    title_ru: String,
    title_en: Option<String>,
    flag: String,
    is_regexp: bool,
    place: i32,
    points: i32,
    keys_reward: Vec<Vec<i32>>,
    keys_condition: Vec<Vec<i32>>,
    author: String,
    character: i32,
    tags: String,
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

pub fn load_tasks_from_repo(url: &str, save_path: &str) -> Fallible<(Vec<InsertTask>, PathBuf)> {
    let repo = Repository::clone(url, save_path)?;
    load_tasks_from_path(repo.path().to_str().unwrap())
}

pub fn load_tasks_from_path(tasks_repo: &str) -> Fallible<(Vec<InsertTask>, PathBuf)> {
    let tasks_repo = PathBuf::from(tasks_repo);
    if tasks_repo.is_dir() {
        let mut tasks: Vec<InsertTask> = vec![];
        let mut map = PathBuf::new();
        for entry in read_dir(tasks_repo)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                tasks.push(load_task(&path)?);
            } else if path.ends_with("map.toml") {
                map = path;
            } else {
                continue;
            }
        }
        Ok((tasks, map))
    } else {
        bail!("Error, while parsing tasks. Path isn't dir!!")
    }
}

pub fn map_getter(map_path: &PathBuf) -> Fallible<Map> {
    Ok(from_str(&read_to_string(map_path)?)?)
}

fn load_task(task_path: &PathBuf) -> Fallible<InsertTask> {
    let mut task: ImportTask = ImportTask::default();
    let mut desc_ru = String::new();
    let mut desc_en = None;
    for entry in read_dir(task_path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() {
            if path.ends_with("task.toml") {
                task = from_str(&read_to_string(path)?)?;
                if !task.is_regexp {
                    task.flag = task.flag.escape_debug().to_string();
                }
            } else if path.ends_with("desc_ru.html") {
                desc_ru = read_to_string(path)?;
            } else if path.ends_with("desc_en.html") {
                desc_en = Some(read_to_string(path)?);
            } else {
                continue;
            }
        } else {
            bail!("Error, while parsing task {:?}. Path isn't dir", task_path)
        }
    }
    info!("Imported task: {} is ok", task.title_ru);
    let keys_reward = vec![1; 6]
        .into_iter()
        .enumerate()
        .map(|(i, _c)| {
            if let Some(p) = task.keys_reward.iter().position(|s| s[0] == i as i32) {
                task.keys_reward[p][1]
            } else {
                1
            }
        })
        .collect();
    let keys_condition = vec![1; 6]
        .iter()
        .enumerate()
        .map(|(i, _c)| {
            if let Some(p) = task.keys_condition.iter().position(|s| s[0] == i as i32) {
                task.keys_condition[p][1]
            } else {
                1
            }
        })
        .collect();

    Ok(InsertTask {
        title_ru: task.title_ru,
        title_en: task.title_en,
        description_ru: desc_ru,
        description_en: desc_en,
        flag: task.flag,
        points: task.points,
        keys_reward,
        keys_condition,
        place: task.place,
        author: task.author,
        character: task.character,
        tags: task.tags,
    })
}
