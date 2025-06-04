use soroban_sdk::{Address, Env, Symbol, Map};

use crate::types::{EducatorStats, TipHistory};

// Storage keys
fn get_admin_key(env: &Env) -> Symbol {
    Symbol::new(env, "ADMIN")
}

fn get_educator_stats_key(env: &Env, _educator: &Address) -> Symbol {
    Symbol::new(env, "EDU_STATS")
}

fn get_tip_history_key(env: &Env, _educator: &Address) -> Symbol {
    Symbol::new(env, "TIP_HIST")
}

fn get_top_educators_key(env: &Env) -> Symbol {
    Symbol::new(env, "TOP_EDU")
}

// Admin management
pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&get_admin_key(env))
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&get_admin_key(env), admin);
}

// Educator stats management
pub fn get_educator_stats(env: &Env, educator: &Address) -> Option<EducatorStats> {
    env.storage().instance().get(&get_educator_stats_key(env, educator))
}

pub fn set_educator_stats(env: &Env, educator: &Address, stats: &EducatorStats) {
    env.storage().instance().set(&get_educator_stats_key(env, educator), stats);
}

// Tip history management
pub fn get_tip_history(env: &Env, educator: &Address) -> Option<TipHistory> {
    env.storage().instance().get(&get_tip_history_key(env, educator))
}

pub fn set_tip_history(env: &Env, educator: &Address, history: &TipHistory) {
    env.storage().instance().set(&get_tip_history_key(env, educator), history);
}

// Top educators management
pub fn get_top_educators(env: &Env) -> Map<Address, EducatorStats> {
    env.storage().instance().get(&get_top_educators_key(env)).unwrap_or(Map::new(env))
}

pub fn set_top_educators(env: &Env, educators: &Map<Address, EducatorStats>) {
    env.storage().instance().set(&get_top_educators_key(env), educators);
}

pub fn update_top_educators(env: &Env, educator: &Address, stats: &EducatorStats) {
    let mut top_educators = get_top_educators(env);
    top_educators.set(educator.clone(), stats.clone());
    set_top_educators(env, &top_educators);
}