use std::collections::HashMap;
use std::hash::Hash;

pub trait KeyValueStore {
    type Key;
    type Value;

    fn set(&mut self, key: Self::Key, value: Self::Value);
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
}

pub struct InMemoryStore<T, U> {
    pub storage: HashMap<T, U>,
}

impl<T, U> KeyValueStore for InMemoryStore<T, U> 
where
    T: Hash + Eq,
{
    type Key = T;
    type Value = U;

    fn set(&mut self, key: T, value: U) {
        self.storage.insert(key, value);
    }

    fn get(&self, key: &T) -> Option<&U> {
        self.storage.get(key)
    }
}

// Example usage
pub fn main() {
    let mut store: InMemoryStore<String, String> = InMemoryStore {
        storage: HashMap::new(),
    };

    store.set("name".to_string(), "Rust".to_string());
    assert_eq!(store.get(&"name".to_string()), Some(&"Rust".to_string()));

    store.set("language".to_string(), "Rust".to_string());
    assert_eq!(
        store.get(&"language".to_string()),
        Some(&"Rust".to_string())
    );

    assert_eq!(store.get(&"non_existent".to_string()), None);
}

