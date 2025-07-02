use soroban_sdk::{contracttype, String, Vec};

use crate::DifficultyLevel;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Content {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub subject_tags: Vec<String>,
    pub content_url: String,
    pub author: Option<String>,
    pub difficulty_level: Option<DifficultyLevel>,
    pub creation_date: Option<u64>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentList {
    pub contents: Vec<Content>,
}