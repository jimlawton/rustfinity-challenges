use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    // TODO: implement this function
    let section_list = get_animals_in_section(section, registry);
    if !section_list.contains(&animal.to_string()) {
        let mut new_val = vec![animal.to_string()];
        registry
            .entry(section.to_string())
            .and_modify(|v| { v.append(&mut new_val) })
            .or_insert(new_val);
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    let mut results: Vec<String> = match registry.get(section) {
        Some(v) => v.to_vec(),
        None => Vec::new(),
    };
    results.sort();
    results
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    // TODO: implement this function
    let sections = registry.keys();
    let mut results: Vec<String> = Vec::new();
    for section in sections {
        let mut values = get_animals_in_section(section, registry);
        if values.len() > 0 {
            results.append(&mut values);
        }
    }
    results.sort();
    results
}

