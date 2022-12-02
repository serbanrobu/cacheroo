use std::{borrow::Borrow, collections::HashMap, hash::Hash, sync::Arc, time::Duration};
use tokio::{
    sync::RwLock,
    task::{self, JoinHandle},
    time,
};

/// A wrapper around [`HashMap`][std::collections::HashMap] so performance characteristics and
/// memory usage are similar. It uses [`RwLock`][tokio::sync::RwLock] for the locking mechanism and
/// [`Arc`][std::sync::Arc] for sharing between threads. In addition to the inserted value, an
/// optional [`JoinHandle`][tokio::task::JoinHandle] is also stored to be able to cancel the key
/// expiration task.
#[derive(Clone)]
pub struct Store<K, V> {
    shared: Arc<RwLock<HashMap<K, Value<V>>>>,
}

type Expiration = JoinHandle<()>;

struct Value<V> {
    inner: V,
    expiration: Option<Expiration>,
}

impl<V> Value<V> {
    fn abort_expiration(self) -> V {
        if let Some(expiration) = self.expiration {
            expiration.abort();
        }

        self.inner
    }
}

impl<K, V> Store<K, V>
where
    K: Eq + Hash,
{
    /// Creates an empty `HashMap`.
    ///
    /// The hash map is initially created with a capacity of 0, so it will not allocate until it is
    /// first inserted into.
    pub fn new() -> Self {
        Self {
            shared: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Returns a cloned value corresponding to the key.
    /// Time: O(1)
    pub async fn get<Q: ?Sized>(&self, k: &Q) -> Option<V>
    where
        Q: Eq + Hash,
        K: Borrow<Q>,
        V: Clone,
    {
        let state = self.shared.read().await;
        state.get(k).map(|v| v.inner.clone())
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the
    /// map.
    /// Time: O(1)
    pub async fn remove<Q: ?Sized>(&self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let mut state = self.shared.write().await;

        state.remove(k).map(Value::abort_expiration)
    }

    /// Inserts a key-value pair into the map, returning the value at the key if the key was
    /// previously in the map.
    /// Time: O(1)
    pub async fn insert(&self, k: K, v: V) -> Option<V> {
        self.insert_with_expiration(k, v, None).await
    }

    /// Inserts a key-value pair into the map with a specified lifetime, returning the value at the
    /// key if the key was previously in the map.
    /// Time: O(1)
    pub async fn insert_with_lifetime(&self, k: K, v: V, lifetime: Duration) -> Option<V>
    where
        K: Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        let shared = self.shared.clone();
        let k_1 = k.clone();

        let handle = task::spawn(async move {
            time::sleep(lifetime).await;
            let mut state = shared.write().await;
            state.remove(&k_1);
        });

        self.insert_with_expiration(k, v, Some(handle)).await
    }

    /// Returns `true` if the map contains a value for the specified key.
    /// Time: O(1)
    pub async fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.shared.read().await.contains_key(k)
    }

    /// Returns the number of elements in the map.
    /// Time: O(1)
    pub async fn len(&self) -> usize {
        self.shared.read().await.len()
    }

    async fn insert_with_expiration(
        &self,
        k: K,
        v: V,
        expiration: Option<Expiration>,
    ) -> Option<V> {
        let mut state = self.shared.write().await;

        let value = Value {
            inner: v,
            expiration,
        };

        state.insert(k, value).map(Value::abort_expiration)
    }
}
