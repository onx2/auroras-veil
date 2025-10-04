use crate::stdb::SubscriptionHandle;
use bevy::{platform::collections::HashMap, prelude::*};
use spacetimedb_sdk::SubscriptionHandle as SubscriptionHandleTrait;

#[derive(Resource, Default)]
pub struct StdbSubscriptions {
    data: HashMap<String, SubscriptionHandle>,
}

impl StdbSubscriptions {
    /// Insert or replace the subscription for the given key. If a previous
    /// subscription exists, it will be unsubscribed before being replaced.
    pub fn upsert(&mut self, key: impl Into<String>, handle: SubscriptionHandle) {
        let key = key.into();
        println!("Subscription tracked: {}", key.clone());
        if let Some(existing) = self.data.insert(key, handle) {
            let _res = existing.unsubscribe();
        }
    }

    /// Remove the subscription for the given key. If it exists, it will be
    /// unsubscribed before being removed. Returns true if a value was removed.
    pub fn remove(&mut self, key: &str) -> bool {
        if let Some(handle) = self.data.remove(key) {
            let _res = <SubscriptionHandle>::unsubscribe(handle);
            println!("Subscription untracked: {}", key);
            true
        } else {
            false
        }
    }

    /// Unsubscribe all active subscriptions and clear the collection.
    pub fn unsubscribe_all(&mut self) {
        for (_, handle) in self.data.drain() {
            let _res = <SubscriptionHandle>::unsubscribe(handle);
        }
    }

    /// Returns true if the collection contains a subscription for the key.
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Number of active subscriptions tracked.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if no subscriptions are tracked.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Drop for StdbSubscriptions {
    fn drop(&mut self) {
        // Ensure we don't leak active subscriptions on shutdown or world teardown.
        for (_, handle) in self.data.drain() {
            let _res = <SubscriptionHandle>::unsubscribe(handle);
        }
    }
}
