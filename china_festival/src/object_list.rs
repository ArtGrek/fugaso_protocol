use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct ObjectsList<T> {
    pub items: Arc<Mutex<HashMap<String, Arc<Mutex<T>>>>>,
}

impl<T> ObjectsList<T> {
    pub fn new() -> Self {
        ObjectsList {
            items: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
