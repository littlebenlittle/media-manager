use js_sys::TryFromIntError;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct ID(String);

impl From<String> for ID {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <String as std::fmt::Display>::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection<T>(HashMap<ID, T>);

impl<T> Collection<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get(&self, id: &ID) -> Option<&T> {
        self.0.get(id)
    }
}

impl<T> Default for Collection<T> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<T> IntoIterator for Collection<T> {
    type Item = <HashMap<ID, T> as IntoIterator>::Item;
    type IntoIter = <HashMap<ID, T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> FromIterator<(ID, T)> for Collection<T> {
    fn from_iter<I: IntoIterator<Item = (ID, T)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}
