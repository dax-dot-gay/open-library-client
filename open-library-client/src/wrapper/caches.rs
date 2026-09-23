//! Request & data caching

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
    sync::{Arc, LazyLock},
    time::Duration,
};

use cached::Cached;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;

static CACHE_STORE: LazyLock<Arc<RwLock<HashMap<String, StoredCache>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

#[derive(Clone, Debug)]
enum StoredCache {
    Unbound(cached::UnboundCache<String, Value>),
    Lru(cached::LruCache<String, Value>),
    Ttl(cached::TtlCache<String, Value>),
    LruTtl(cached::LruTtlCache<String, Value>),
    TtlSorted(cached::TtlSortedCache<String, Value>),
}

/// The type of an underlying cache.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum CacheKind {
    Unbound,
    Lru,
    Ttl,
    LruTtl,
    TtlSorted,
}

fn store_cache(name: String, cache: StoredCache) -> crate::Result<()> {
    let mut store = CACHE_STORE.write();
    if store.contains_key(&name) {
        return Err(crate::Error::cache_exists(name));
    }

    let _ = store.insert(name, cache);
    Ok(())
}

macro_rules! CacheBuilder {
    ($($(#[$root_meta:meta])+)? $method_name:ident => $cache_variant:ident($cache_type:ident) {
        $($($(#[$field_meta:meta])+)? $field_name:ident: $field_type:ty),*
    }) => {
        #[bon::bon]
        impl<S: Cacheable> Cache<S> {
            $($(#[$root_meta])+)?
            #[builder(finish_fn = build)]
            pub fn $method_name(
                #[builder(start_fn, into)] cache_name: String,
                $($($(#[$field_meta])+)? $field_name: Option<$field_type>),*
            ) -> crate::Result<Self> {
                let mut builder = cached::$cache_type::<String, serde_json::Value>::builder();
                $(
                    if let Some(_val) = $field_name {
                        builder = builder.$field_name(_val);
                    }
                )*
                let built = StoredCache::$cache_variant(builder.build()?);
                store_cache(cache_name.clone(), built)?;
                Ok(Self {
                    reference: cache_name,
                    _s: PhantomData
                })
            }
        }
    };
}

/// A reference to a named cache in the global store
#[derive(Clone, Debug)]
pub struct Cache<S: Cacheable> {
    reference: String,
    _s: PhantomData<S>,
}

impl<S: Cacheable> Cache<S> {
    /// Create a reference to an existing cache
    pub fn new(name: impl Into<String>) -> Option<Self> {
        let name = name.into();
        CACHE_STORE.read().get(&name).map(|_| Self {
            reference: name,
            _s: Default::default(),
        })
    }

    /// Create a reference to a cache, creating the cache with a closure if it doesn't exist
    pub fn new_or(
        name: impl Into<String>,
        init: impl Fn() -> Result<Self, Box<dyn Error>>,
    ) -> Self {
        let name = name.into();
        if CACHE_STORE.read().contains_key(&name) {
            Self {
                reference: name,
                _s: Default::default(),
            }
        } else {
            init().expect(
                "Failed to initialize from initializer function. This indicates user error.",
            )
        }
    }
}

/*
let mut stores = CACHE_STORE.write();
let mut store = stores.get_mut(&self.name()).unwrap();
let result = match store {
    StoredCache::Unbound(cache) => todo!(),
    StoredCache::Lru(cache) => todo!(),
    StoredCache::Ttl(cache) => todo!(),
    StoredCache::LruTtl(cache) => todo!(),
    StoredCache::TtlSorted(cache) => todo!(),
};
*/

impl<S: Cacheable> Cache<S> {
    /// Get the name of this cache
    pub fn name(&self) -> String {
        self.reference.clone()
    }

    /// Attempt to get a value from this cache
    pub fn get(&self, key: impl Display) -> crate::Result<Option<S>> {
        let key = key.to_string();
        let mut stores = CACHE_STORE.write();
        let store = stores.get_mut(&self.name()).unwrap();
        let result = match store {
            StoredCache::Unbound(cache) => cache.cache_get(&key),
            StoredCache::Lru(cache) => cache.cache_get(&key),
            StoredCache::Ttl(cache) => cache.cache_get(&key),
            StoredCache::LruTtl(cache) => cache.cache_get(&key),
            StoredCache::TtlSorted(cache) => cache.cache_get(&key),
        }
        .cloned();
        if let Some(r) = result {
            Ok(Some(serde_json::from_value::<S>(r)?))
        } else {
            Ok(None)
        }
    }

    /// Check if a given key exists in the cache
    pub fn contains(&self, key: impl Display) -> bool {
        let key = key.to_string();
        let mut stores = CACHE_STORE.write();
        let store = stores.get_mut(&self.name()).unwrap();
        match store {
            StoredCache::Unbound(cache) => cache.cache_contains(&key),
            StoredCache::Lru(cache) => cache.cache_contains(&key),
            StoredCache::Ttl(cache) => cache.cache_contains(&key),
            StoredCache::LruTtl(cache) => cache.cache_contains(&key),
            StoredCache::TtlSorted(cache) => cache.cache_contains(&key),
        }
    }

    /// Insert an item into cache. This will silently replace existing entries.
    pub fn set(&self, item: impl Cacheable) -> crate::Result<()> {
        let key = item.key();
        let serialized = serde_json::to_value(item)?;
        let mut stores = CACHE_STORE.write();
        let store = stores.get_mut(&self.name()).unwrap();
        let _ = match store {
            StoredCache::Unbound(cache) => cache.cache_set(key, serialized),
            StoredCache::Lru(cache) => cache.cache_set(key, serialized),
            StoredCache::Ttl(cache) => cache.cache_set(key, serialized),
            StoredCache::LruTtl(cache) => cache.cache_set(key, serialized),
            StoredCache::TtlSorted(cache) => cache.cache_set(key, serialized),
        };
        Ok(())
    }

    /// Insert an item into the cache. If an item with the value's key already exists, replace it and return it
    pub fn insert(&self, item: impl Cacheable) -> crate::Result<Option<S>> {
        let key = item.key();
        let serialized = serde_json::to_value(item)?;
        let mut stores = CACHE_STORE.write();
        let store = stores.get_mut(&self.name()).unwrap();
        let existing = match store {
            StoredCache::Unbound(cache) => cache.cache_set(key, serialized),
            StoredCache::Lru(cache) => cache.cache_set(key, serialized),
            StoredCache::Ttl(cache) => cache.cache_set(key, serialized),
            StoredCache::LruTtl(cache) => cache.cache_set(key, serialized),
            StoredCache::TtlSorted(cache) => cache.cache_set(key, serialized),
        };
        if let Some(ex) = existing {
            Ok(Some(serde_json::from_value::<S>(ex)?))
        } else {
            Ok(None)
        }
    }

    /// Removes an item from the cache if it exists. Returns true if the item existed and was deleted.
    pub fn remove(&self, key: impl Display) -> bool {
        let key = key.to_string();
        let mut stores = CACHE_STORE.write();
        let store = stores.get_mut(&self.name()).unwrap();
        match store {
            StoredCache::Unbound(cache) => cache.cache_remove(&key),
            StoredCache::Lru(cache) => cache.cache_remove(&key),
            StoredCache::Ttl(cache) => cache.cache_remove(&key),
            StoredCache::LruTtl(cache) => cache.cache_remove(&key),
            StoredCache::TtlSorted(cache) => cache.cache_remove(&key),
        }
        .is_some()
    }

    /// Removes a value from the cache, returning it. This may fail after the removal completes.
    pub fn take(&self, key: impl Display) -> crate::Result<Option<S>> {
        let key = key.to_string();
        let mut stores = CACHE_STORE.write();
        let store = stores.get_mut(&self.name()).unwrap();
        let result = match store {
            StoredCache::Unbound(cache) => cache.cache_remove(&key),
            StoredCache::Lru(cache) => cache.cache_remove(&key),
            StoredCache::Ttl(cache) => cache.cache_remove(&key),
            StoredCache::LruTtl(cache) => cache.cache_remove(&key),
            StoredCache::TtlSorted(cache) => cache.cache_remove(&key),
        };
        if let Some(r) = result {
            Ok(Some(serde_json::from_value::<S>(r)?))
        } else {
            Ok(None)
        }
    }

    /// Clear this cache. Does not trigger on_evict callbacks
    pub fn clear(&self) -> () {
        let mut stores = CACHE_STORE.write();
        let store = stores.get_mut(&self.name()).unwrap();
        let _ = match store {
            StoredCache::Unbound(cache) => cache.cache_reset(),
            StoredCache::Lru(cache) => cache.cache_reset(),
            StoredCache::Ttl(cache) => cache.cache_reset(),
            StoredCache::LruTtl(cache) => cache.cache_reset(),
            StoredCache::TtlSorted(cache) => cache.cache_reset(),
        };
    }

    /// Evict expired results. Does not affect unbounded caches and lru caches
    pub fn evict(&self) -> usize {
        let mut stores = CACHE_STORE.write();
        let store = stores.get_mut(&self.name()).unwrap();
        match store {
            StoredCache::Unbound(_) => 0,
            StoredCache::Lru(_) => 0,
            StoredCache::Ttl(cache) => cache.evict(),
            StoredCache::LruTtl(cache) => cache.evict(),
            StoredCache::TtlSorted(cache) => cache.evict(),
        }
    }

    /// Clear this cache, triggering on_evict on all items cleared.
    pub fn clear_with_on_evict(&self) -> () {
        let mut stores = CACHE_STORE.write();
        let store = stores.get_mut(&self.name()).unwrap();
        let _ = match store {
            StoredCache::Unbound(cache) => cache.cache_clear_with_on_evict(),
            StoredCache::Lru(cache) => cache.cache_clear_with_on_evict(),
            StoredCache::Ttl(cache) => cache.cache_clear_with_on_evict(),
            StoredCache::LruTtl(cache) => cache.cache_clear_with_on_evict(),
            StoredCache::TtlSorted(cache) => cache.cache_clear_with_on_evict(),
        };
    }

    /// Return this cache's size
    pub fn size(&self) -> usize {
        let stores = CACHE_STORE.read();
        let store = stores.get(&self.name()).unwrap();
        match store {
            StoredCache::Unbound(cache) => cache.cache_size(),
            StoredCache::Lru(cache) => cache.cache_size(),
            StoredCache::Ttl(cache) => cache.cache_size(),
            StoredCache::LruTtl(cache) => cache.cache_size(),
            StoredCache::TtlSorted(cache) => cache.cache_size(),
        }
    }

    /// Get the type of the underlying cache
    pub fn kind(&self) -> CacheKind {
        let stores = CACHE_STORE.read();
        let store = stores.get(&self.name()).unwrap();
        match store {
            StoredCache::Unbound(_) => CacheKind::Unbound,
            StoredCache::Lru(_) => CacheKind::Lru,
            StoredCache::Ttl(_) => CacheKind::Ttl,
            StoredCache::LruTtl(_) => CacheKind::LruTtl,
            StoredCache::TtlSorted(_) => CacheKind::TtlSorted,
        }
    }
}

CacheBuilder!(
    /// Create an unbounded cache
    unbound => Unbound(UnboundCache) {
        initial_capacity: usize,
        on_evict: impl Fn(&String, &serde_json::Value) + Send + Sync + 'static
    }
);

CacheBuilder!(
    /// Create a LRU cache
    lru => Lru(LruCache) {
        max_size: usize,
        on_evict: impl Fn(&String, &serde_json::Value) + Send + Sync + 'static
    }
);

CacheBuilder!(
    /// Create a TTL cache
    ttl => Ttl(TtlCache) {
        ttl: Duration,
        initial_capacity: usize,
        refresh_on_hit: bool,
        on_evict: impl Fn(&String, &serde_json::Value) + Send + Sync + 'static
    }
);

CacheBuilder!(
    /// Create a LRU-TTL cache
    lru_ttl => LruTtl(LruTtlCache) {
        max_size: usize,
        ttl: Duration,
        refresh_on_hit: bool
    }
);

CacheBuilder!(
    /// Created a sorted TTL cache
    ttl_sorted => TtlSorted(TtlSortedCache) {
        max_size: usize,
        initial_capacity: usize,
        ttl: Duration,
        on_evict: impl Fn(&String, &serde_json::Value) + Send + Sync + 'static
    }
);

/// An item that can be added to or retrieved from a cache
pub trait Cacheable: Clone + Debug + Serialize + DeserializeOwned {
    /// Return the key of this instance (should be unique, will be used to retrieve from the cache later)
    fn key(&self) -> String;
}
