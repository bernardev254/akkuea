use crate::error::Error;
use crate::metadata::Content;
use crate::storage::ContentStorage;
use soroban_sdk::{Env, String as SorobanString, Vec};

/// Core search functionality for educational content with indexed optimization
pub fn search_content(env: &Env, subject: SorobanString) -> Result<Vec<Content>, Error> {
    // Try indexed search first - O(1) + O(m) complexity where m is matching items
    let indexed_results = ContentStorage::search_content_by_tag_indexed(env, &subject);

    if !indexed_results.is_empty() {
        return Ok(indexed_results);
    }

    // Check if we have any content at all - if no content exists, return error
    let all_content = ContentStorage::get_all_content(env);
    if all_content.is_empty() {
        return Err(Error::NoMatchingContent);
    }

    // If indexed search returns empty but content exists, this means the tag truly doesn't match
    // Don't fall back to linear search for exact tag matches to maintain consistency
    Err(Error::NoMatchingContent)
}

/// Linear search implementation (original algorithm) for fallback
fn search_content_linear(env: &Env, subject: SorobanString) -> Result<Vec<Content>, Error> {
    let contents = ContentStorage::get_all_content(env);
    let mut results = Vec::new(env);

    for content in contents.iter() {
        // Check if the content has the searched tag
        let mut found = false;
        for tag in content.subject_tags.iter() {
            if tag == subject {
                found = true;
                break;
            }
        }

        if found {
            results.push_back(content.clone());
        }
    }

    if results.is_empty() {
        Err(Error::NoMatchingContent)
    } else {
        Ok(results)
    }
}

/// Batch search for multiple tags using indexed search - efficient for complex queries
pub fn search_content_multi_tag(
    env: &Env,
    tags: Vec<SorobanString>,
) -> Result<Vec<Content>, Error> {
    let mut all_results = Vec::new(env);
    let mut seen_ids = Vec::new(env);

    for tag in tags.iter() {
        let tag_results = ContentStorage::search_content_by_tag_indexed(env, &tag);

        for content in tag_results.iter() {
            // Check if we've already added this content (avoid duplicates)
            let mut already_added = false;
            for seen_id in seen_ids.iter() {
                if seen_id == content.id {
                    already_added = true;
                    break;
                }
            }

            if !already_added {
                seen_ids.push_back(content.id);
                all_results.push_back(content.clone());
            }
        }
    }

    if all_results.is_empty() {
        Err(Error::NoMatchingContent)
    } else {
        Ok(all_results)
    }
}
