use soroban_sdk::{contracttype, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Content {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub subject_tags: Vec<String>,
    pub content_url: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentList {
    pub contents: Vec<Content>,
}