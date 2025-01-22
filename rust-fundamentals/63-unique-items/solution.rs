use std::collections::HashSet;

pub fn unique_items<I, T>(items: I) -> Vec<String> 
where 
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let mut unique_set: HashSet<String> = HashSet::new();
    let mut result = items
        .filter_map(|item| {
            let trimmed_item = item.as_ref().trim().to_string();
            if trimmed_item.is_empty() {
                None
            } else {
                if unique_set.insert(trimmed_item.clone()) { 
                    Some(trimmed_item)
                } else {
                    None
                }
            }
        })
        .collect::<Vec<String>>();
    result.sort();
    result
}

/// Example usage
pub fn main() {
    let product_ids = vec![
        "abc123".to_string(),
        "  ".to_string(),
        "def456".to_string(),
        "abc123".to_string(),
        "ghi789".to_string(),
        "ghi789".to_string(),
        "   def456".to_string(),
    ];

    let unique_ids = unique_items(product_ids.into_iter());
    assert_eq!(unique_ids, vec!["abc123", "def456", "ghi789"]);
}

