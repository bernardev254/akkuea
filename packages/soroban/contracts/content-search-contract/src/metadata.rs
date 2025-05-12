use soroban_sdk::{contracttype, Env, String as SorobanString, Vec, Symbol, symbol_short};

const CONTENT_KEY: Symbol = symbol_short!("CONTENT");
const NEXT_ID_KEY: Symbol = symbol_short!("NEXT_ID");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Content {
    pub id: u64,
    pub title: SorobanString,
    pub description: SorobanString,
    pub subject_tags: Vec<SorobanString>,
    pub content_url: SorobanString,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContentList {
    pub contents: Vec<Content>,
}

pub struct ContentStorage;

impl ContentStorage {
    pub fn get_content(env: &Env, id: u64) -> Option<Content> {
        let storage = env.storage().instance();
        let content_list: ContentList = storage.get(&CONTENT_KEY).unwrap_or(ContentList {
            contents: Vec::new(env),
        });
        content_list.contents.iter().find(|c| c.id == id).map(|c| c.clone())
    }

    pub fn set_content(env: &Env, content: &Content) {
        let storage = env.storage().instance();
        let mut content_list: ContentList = storage.get(&CONTENT_KEY).unwrap_or(ContentList {
            contents: Vec::new(env),
        });
        
        // Actualizar o añadir el contenido
        let mut found = false;
        for i in 0..content_list.contents.len() {
            if content_list.contents.get_unchecked(i).id == content.id {
                content_list.contents.set(i, content.clone());
                found = true;
                break;
            }
        }
        
        if !found {
            content_list.contents.push_back(content.clone());
        }
        
        storage.set(&CONTENT_KEY, &content_list);
        storage.extend_ttl(50, 100);
    }

    pub fn get_all_content(env: &Env) -> Vec<Content> {
        let storage = env.storage().instance();
        let content_list: ContentList = storage.get(&CONTENT_KEY).unwrap_or(ContentList {
            contents: Vec::new(env),
        });
        content_list.contents
    }
} 