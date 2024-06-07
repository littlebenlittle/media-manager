//! Storage API for the UI.
//!
//! Storage acts a write-back cache for the main storage on the server.
//! Conflicts between local and main storage are resolved via an
//! agreed-upon resolution algorithm _a la_ [CRDTs]. Or at least they
//! will be wehn I get around to it.
//!
//! [CRDTs]: https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type

use std::{
    borrow::Borrow,
    cell::Cell,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    future::IntoFuture,
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use futures::{channel::mpsc, SinkExt, Stream};
use leptos::{ReadSignal, RwSignal};
use serde::{Deserialize, Deserializer, Serialize};

/// TODO: Use this! Use storage as a KV cache.
pub enum Coherency {
    WriteThrough,
    WriteBack(leptos::Trigger),
}

pub trait StorageTrait {
    // network error or otherwise
    type Err;
    fn list() -> Result<Vec<String>, Self::Err>;
    fn get(id: &ID) -> Result<String, Self::Err>;
    fn put(val: &String) -> Result<(), Self::Err>;
    fn drop(id: &ID) -> Result<(), Self::Err>;
}

pub struct Cache<C, M>
where
    C: StorageTrait,
    M: StorageTrait,
{
    cache: C,
    main: M,
    coherency: Coherency,
}

pub trait CRDT: Clone {
    fn resolve(left: &Self, right: &Self) -> Self;
}

impl<T: CRDT> CRDT for Option<T> {
    fn resolve(left: &Self, right: &Self) -> Self {
        match (left, right) {
            (Some(l), Some(r)) => Some(T::resolve(l, r)),
            (Some(l), None) => Some(l.clone()),
            (None, Some(r)) => Some(r.clone()),
            (None, None) => None,
        }
    }
}

// impl CRDT for Metadata {
//     fn resolve(left: &Self, right: &Self) -> Self {
//         if left.timestamp > right.timestamp {
//             left.clone()
//         } else {
//             right.clone()
//         }
//     }
// }

/// A doc is a nested KV store, kind of like a simplified JSON object
/// where the only atomic type is `string` and the only composite type
/// is `map`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Doc {
    Val(String),
    Map(BTreeMap<String, Doc>),
}

impl Doc {
    pub fn get<'a>(&'a self, path: impl AsRef<str>) -> Option<&'a Doc> {
        let path = path.as_ref();
        match self {
            Self::Val(_) => {
                if path == "" {
                    Some(self)
                } else {
                    None
                }
            }
            Self::Map(ref map) => {
                if let Some((key, rest)) = path.split_once(".") {
                    match map.get(key) {
                        Some(d) => d.get(rest),
                        None => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Map(l), Self::Map(r)) => {
                let mut map = BTreeMap::new();
                for (k, lv) in l.iter() {
                    if let Some(rv) = r.get(k) {
                        map.insert(k.clone(), lv.merge(rv));
                    } else {
                        map.insert(k.clone(), lv.clone());
                    }
                }
                Doc::Map(map)
            }
            _ => self.clone(),
        }
    }

    pub fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(s)?)
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Unwraps the `Val` arm of the enum. Panics if this
    /// Doc is not a Val.
    pub fn into_val<'a>(&'a self) -> &'a str {
        match self {
            Self::Val(val) => &val,
            _ => panic!("`into_val` called on a Map"),
        }
    }
}

use crate::{client::Media, data::Metadata, log};

/// IDs are base64 encoded sha256 hashes.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct ID(String);

impl ID {
    pub fn gen(v: &str) -> Self {
        use base64::prelude::*;
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(v);
        let result = hasher.finalize();
        let id = BASE64_STANDARD.encode(result);
        return Self(id);
    }

    pub fn as_str<'a>(&'a self) -> &'a str {
        &self.0
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ID {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<ID> for String {
    fn from(value: ID) -> String {
        value.0
    }
}

impl<'a> From<&'a ID> for &'a str {
    fn from(value: &'a ID) -> &'a str {
        &value.0
    }
}

impl From<&str> for ID {
    fn from(value: &str) -> Self {
        ID(value.to_owned())
    }
}

impl leptos::IntoView for &ID {
    fn into_view(self) -> leptos::View {
        (&self.0).into_view()
    }
}

impl leptos::IntoView for ID {
    fn into_view(self) -> leptos::View {
        (&self.0).into_view()
    }
}

pub enum MediaEvent {
    Create(Media),
    Delete(Media),
}

#[inline(always)]
fn get_storage() -> web_sys::Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

/// Returns a list of media items in the store.
pub fn list() -> impl Iterator<Item = Doc> {
    let storage = get_storage();
    ids().into_iter().filter_map(move |id| {
        storage
            .get_item(id.as_str())
            .unwrap()
            .map(|s| Doc::from_str(&s).ok())?
    })
}

/// Returns the value if the provided ID is present in local storage.
pub fn get(id: &ID) -> Option<Doc> {
    match get_storage().get_item(id.into()) {
        Err(e) => {
            log!(e.as_string().unwrap());
            None
        }
        Ok(None) => None,
        Ok(Some(s)) => match Doc::from_str(&s) {
            Err(e) => {
                log!(e);
                None
            }
            Ok(doc) => Some(doc),
        },
    }
}

/// Inserts a value into the collection and returns the
/// generated ID.
pub fn put(data: &Doc) -> ID {
    let value = data.to_string();
    let id = ID::gen(&value);
    insert(&id, &value);
    return id;
}

/// Updates a value in the collection using the overlay
/// strategy. Does nothing if ID is not present.
pub fn update(id: &ID, doc: &Doc) {
    if let Some(orig) = get(id) {
        put(&orig.merge(doc));
    }
}

/// Drops a value from the collection. Does nothing if ID
/// is not present.
pub fn drop(id: &ID) {
    if let Err(e) = get_storage().remove_item(id.into()) {
        log!(e.as_string().unwrap())
    }
}

/// Synchronize collection with main storage.
pub async fn sync() {
    use crate::client::{media_share, media_sync, MediaSyncResponse as Res};
    match media_sync(&ids()).await {
        Err(e) => log!(e),
        Ok(Res { have, want }) => {
            let share = want.iter().filter_map(|id| get(id)).collect();
            media_share(&share);
            for media in have.iter() {
                put(media);
            }
        }
    }
}

#[inline(always)]
fn insert(id: &ID, value: &str) {
    get_storage().set_item(&id.0, value);
}

fn ids() -> Vec<ID> {
    let len = get_storage().length().unwrap() as usize;
    let mut i = 0;
    let mut ids = Vec::with_capacity(len);
    while i < len {
        ids[i] = ID(get_storage().key(i as u32).unwrap().unwrap());
        i += 1;
    }
    return ids;
}

/// Returns the value if the provided ID is present in local storage.
// pub async fn url_of(id: &ID) -> Option<String> {
//     if let Some(url) = get_storage().get_item(id.into()).unwrap() {
//         return Some(url);
//     } else {
//         crate::client::url_of(id).await
//     }
// }

pub trait Storage: Clone {
    type Iter<'a>: Iterator<Item = (String, String)>
    where
        Self: 'a;
    fn get_item(&self, key: &str) -> Option<String>;
    fn set_item(&self, key: &str, val: &str);
    fn remove_item(&self, key: &str);
    fn has_key(&self, key: &str) -> bool;
    fn iter(&self) -> Self::Iter<'_>;
}

/// Simple wrapper around `web_sys::Storage`
#[derive(Clone)]
pub struct LocalStorage {
    storage: web_sys::Storage,
    //TODO profile performance with HashSet if you ever get bored
    keys: BTreeSet<String>,
}

impl Default for LocalStorage {
    fn default() -> Self {
        let storage = get_storage();
        let cap = storage.length().unwrap();
        let mut keys = BTreeSet::new();
        for i in 0..cap {
            let key = storage.key(i).unwrap().unwrap();
            keys.insert(key);
        }
        Self { storage, keys }
    }
}

impl LocalStorage {
    fn get_item(&self, key: &str) -> Option<String> {
        self.storage.get_item(key).unwrap()
    }
    fn set_item(&self, key: &str, val: &str) {
        self.storage.set_item(key, val).unwrap()
    }
    fn remove_item(&self, key: &str) {
        self.storage.remove_item(key).unwrap()
    }
    fn has_key(&self, key: &str) -> bool {
        self.keys.contains(key)
    }
    fn iter<'a>(&'a self) -> impl Iterator<Item = (String, String)> + 'a {
        let cap = self.storage.length().unwrap();
        let x = (0..cap).map(|n| {
            let key = self.storage.key(n).unwrap().unwrap();
            let val = self.storage.get_item(&key).unwrap().unwrap();
            (key, val)
        });
        x
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct KeyValue<'a> {
    key: &'a str,
    val: &'a str,
}

impl<'a> From<(&'a str, &'a str)> for KeyValue<'a> {
    fn from(value: (&'a str, &'a str)) -> Self {
        KeyValue {
            key: value.0,
            val: value.1,
        }
    }
}

pub struct RemoteStorage {
    origin: String,
}

impl Default for RemoteStorage {
    fn default() -> Self {
        Self {
            origin: crate::client::origin(),
        }
    }
}

impl RemoteStorage {
    async fn get_item(&self, key: &str) -> anyhow::Result<Option<String>> {
        let url = format!("{}/api/media/{}", self.origin, key);
        let res = gloo_net::http::Request::get(&url).send().await?;
        if res.status() == 404 {
            return Ok(None);
        }
        if res.status() == 200 {
            return Ok(Some(res.json::<String>().await?));
        }
        anyhow::bail!("request failed");
    }

    async fn set_item(&self, key: &str, val: &str) -> anyhow::Result<()> {
        let url = format!("{}/api/media/{}", self.origin, key);
        let kv = KeyValue::from((key, val));
        let res = gloo_net::http::Request::post(&url)
            .json(&kv)?
            .send()
            .await?;
        if res.status() == 202 {
            return Ok(());
        }
        anyhow::bail!("request failed");
    }

    async fn remove_item(&self, key: &str) -> anyhow::Result<()> {
        let url = format!("{}/api/media/{}", self.origin, key);
        let res = gloo_net::http::Request::delete(&url).send().await?;
        if res.status() == 204 {
            return Ok(());
        }
        anyhow::bail!("request failed");
    }

    async fn has_key(&self, key: &str) -> anyhow::Result<bool> {
        let url = format!("{}/api/media/{}", self.origin, key);
        let res = gloo_net::http::Request::get(&url).send().await?;
        if res.status() == 404 {
            return Ok(false);
        }
        if res.status() == 200 {
            return Ok(true);
        }
        anyhow::bail!("request failed");
    }

    // async fn query() -> anyhow::Result<Vec<(String, String)>> {}
}

/// Hack because I can't figure out how `serde` should work
/// for `StorageEvent<T>`, even when `T` is `serde`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaStorageEvent {
    Assign(ID, Metadata),
    Delete(ID),
}

impl From<StorageEvent<Metadata>> for MediaStorageEvent {
    fn from(value: StorageEvent<Metadata>) -> Self {
        match value {
            StorageEvent::Assign(id, m) => Self::Assign(id, m),
            StorageEvent::Delete(id) => Self::Delete(id),
        }
    }
}

impl From<MediaStorageEvent> for StorageEvent<Metadata> {
    fn from(value: MediaStorageEvent) -> Self {
        match value {
            MediaStorageEvent::Assign(id, m) => Self::Assign(id, m),
            MediaStorageEvent::Delete(id) => Self::Delete(id),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StorageEvent<T>
where
    for<'de> T: Clone + Serialize + Deserialize<'de> + 'static,
{
    Assign(ID, T),
    Delete(ID),
}

impl<T> StorageEvent<T>
where
    for<'de> T: Clone + Serialize + Deserialize<'de> + 'static,
{
    pub fn id<'a>(&'a self) -> &'a ID {
        match self {
            Self::Assign(id, _) => id,
            Self::Delete(id) => id,
        }
    }
}

// TODO can I get away with just an Rc?
#[derive(Clone)]
struct Subs<T>(Arc<Mutex<Vec<mpsc::UnboundedSender<StorageEvent<T>>>>>)
where
    for<'de> T: Clone + Serialize + Deserialize<'de> + 'static;

impl<T> Subs<T>
where
    for<'de> T: Clone + Serialize + Deserialize<'de> + 'static,
{
    fn len(&self) -> usize {
        let subs = self.0.lock().unwrap();
        let len = subs.len();
        return len;
    }

    fn push(&self, tx: mpsc::UnboundedSender<StorageEvent<T>>) {
        let mut subs = self.0.lock().unwrap();
        subs.push(tx);
    }

    fn clone_inner(&self) -> Vec<mpsc::UnboundedSender<StorageEvent<T>>> {
        let subs = self.0.lock().unwrap();
        let ret = subs.clone();
        return ret;
    }
}

impl<T> Default for Subs<T>
where
    for<'de> T: Clone + Serialize + Deserialize<'de> + 'static,
{
    fn default() -> Self {
        Self(Arc::new(Mutex::new(Default::default())))
    }
}

/// A collection of items in the `storage` with the same
/// type.
#[derive(Clone)]
pub struct Collection<T>
where
    for<'de> T: Clone + Serialize + Deserialize<'de> + 'static,
{
    storage: LocalStorage,
    prefix: String,
    subs: Subs<T>,
    _pd: PhantomData<T>,
}

impl<T> Collection<T>
where
    for<'de> T: Clone + Serialize + Deserialize<'de> + 'static,
{
    pub fn new(prefix: &str) -> Self {
        Self {
            storage: Default::default(),
            prefix: prefix.to_owned(),
            subs: Default::default(),
            _pd: PhantomData,
        }
    }

    pub fn get(&self, id: &ID) -> Option<T> {
        let key = self.prefix.clone() + id.as_str();
        if let Some(s) = self.storage.get_item(&key) {
            match serde_json::from_str(s.as_str()) {
                Ok(val) => Some(val),
                Err(e) => {
                    log!(e);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (ID, T)> + 'a {
        self.storage.iter().filter_map(|(k, v)| {
            if k.starts_with(self.prefix.as_str()) {
                match serde_json::from_str(&v) {
                    Ok(t) => Some((k[self.prefix.len()..].into(), t)),
                    Err(e) => {
                        log!(e);
                        None
                    }
                }
            } else {
                None
            }
        })
    }

    pub fn list(&self) -> HashMap<ID, T> {
        self.iter().collect()
    }

    pub fn set(&self, id: &ID, t: &T) {
        let key = self.prefix.clone() + id.as_str();
        if self.subs.len() > 0 {
            if self.storage.has_key(&key) {
                // this is an update
            } else {
                self.notify(StorageEvent::Assign(id.clone(), t.clone()))
            }
        }
        match serde_json::to_string(t) {
            Ok(val) => self.storage.set_item(&key, &val),
            Err(e) => log!(e),
        }
    }

    pub fn drop(&self, id: &ID) {
        if self.subs.len() > 0 {
            if let Some(_) = self.get(id) {
                self.notify(StorageEvent::Delete(id.clone()));
            }
        }
        let key = self.prefix.clone() + id.as_str();
        self.storage.remove_item(&key);
    }

    /// Add a new subscriber. Panics if the number of subs exceeds
    /// `isize::MAX`.
    pub fn subscribe(&self) -> impl Stream<Item = StorageEvent<T>> {
        let (tx, rx) = mpsc::unbounded();
        self.subs.push(tx);
        return rx;
    }

    fn notify(&self, ev: StorageEvent<T>) {
        let mut subs = self.subs.clone_inner();
        leptos::spawn_local(async move {
            for sub in subs.iter_mut() {
                if let Err(e) = sub.send(ev.clone()).await {
                    log!(e)
                }
            }
        })
    }

    pub async fn sync(&self) -> anyhow::Result<()> {
        let url = format!("{}/api/media", crate::client::origin());
        let res = gloo_net::http::Request::get(&url).send().await?;
        if res.status() != 200 {
            anyhow::bail!("request falied")
        }
        let list = res.json::<HashMap<String, T>>().await?;
        for (k, v) in list.iter() {
            self.set(&k.as_str().into(), v)
        }
        Ok(())
    }
}
