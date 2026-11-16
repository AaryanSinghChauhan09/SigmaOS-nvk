// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-nvk: nvk component model.
//!
//! Real implementation: nvk_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NvkItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NvkError {
    EmptyNvkItemName,
    DuplicateNvkItem(String),
    EmptyEventSubject,
}

impl fmt::Display for NvkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyNvkItemName => write!(f, "nvk_item name must not be empty"),
            Self::DuplicateNvkItem(id) => write!(f, "duplicate nvk_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for NvkError {}

pub struct Nvk {
    pub name: String,
    nvk_items: BTreeMap<String, NvkItem>,
    events: BTreeMap<String, Event>,
}

impl Nvk {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nvk_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_nvk_item(&mut self, item: NvkItem) -> Result<(), NvkError> {
        if item.name.trim().is_empty() {
            return Err(NvkError::EmptyNvkItemName);
        }
        let id = item.id.clone();
        if self.nvk_items.contains_key(&id) {
            return Err(NvkError::DuplicateNvkItem(id.clone()));
        }
        self.nvk_items.insert(id, item);
        Ok(())
    }

    pub fn nvk_item_count(&self) -> usize {
        self.nvk_items.len()
    }

    pub fn get_nvk_item(&self, id: &str) -> Option<&NvkItem> {
        self.nvk_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), NvkError> {
        if event.subject.trim().is_empty() {
            return Err(NvkError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_nvk_item(id: &str, name: &str) -> NvkItem {
        NvkItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Nvk::new("Nvk");
        assert_eq!(m.name, "Nvk");
        assert_eq!(m.nvk_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_nvk_item() {
        let mut m = Nvk::new("Nvk");
        m.add_nvk_item(sample_nvk_item("a1", "Alpha")).unwrap();
        assert_eq!(m.nvk_item_count(), 1);
        assert!(m.get_nvk_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Nvk::new("Nvk");
        assert!(m.add_nvk_item(sample_nvk_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Nvk::new("Nvk");
        m.add_nvk_item(sample_nvk_item("x", "One")).unwrap();
        assert!(m.add_nvk_item(sample_nvk_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Nvk::new("Nvk");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
