pub mod engine;
pub mod filters;

// Re-export the main search function for backward compatibility
pub use engine::search_content;