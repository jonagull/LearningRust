use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            done: false,
        }
    }
}