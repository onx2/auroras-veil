#![allow(dead_code)]

use crate::stdb::SubscriptionHandle;
use bevy::{platform::collections::HashMap, prelude::*};
use spacetimedb_sdk::SubscriptionHandle as SubscriptionHandleTrait;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SubKey {
    OwnedCharacterData,
    LocalGameplayData,
    GlobalData,
}

#[derive(Resource, Default)]
pub struct StdbSubscriptions {
    // Each key groups a list of related subscriptions.
    data: HashMap<SubKey, Vec<SubscriptionHandle>>,
}

impl StdbSubscriptions {
    /// Insert the subscription handle into the group for the given key.
    /// If the key does not exist, a new group will be created.
    /// Note: Duplicates are not deduplicated because SubscriptionHandle doesn't implement Eq/Hash.
    pub fn upsert(&mut self, key: SubKey, handle: SubscriptionHandle) {
        let group = self.data.entry(key).or_default();
        group.push(handle);
        println!("Subscription tracked (added to group): {:?}", key);
    }

    /// Replace the entire group for `key` with the provided handles.
    /// Any existing handles in the group will be unsubscribed before replacement.
    pub fn replace<I>(&mut self, key: SubKey, handles: I)
    where
        I: IntoIterator<Item = SubscriptionHandle>,
    {
        let key: SubKey = key.into();
        if let Some(group) = self.data.remove(&key) {
            for handle in group {
                let _ = handle.unsubscribe();
            }
        }

        let new_group: Vec<SubscriptionHandle> = handles.into_iter().collect();
        if !new_group.is_empty() {
            self.data.insert(key, new_group);
        }
    }

    /// Remove the entire subscription group for the given key.
    /// All handles in the group will be unsubscribed before removal.
    /// Returns true if a group existed and was removed.
    pub fn remove(&mut self, key: SubKey) -> bool {
        if let Some(group) = self.data.remove(&key) {
            for handle in group {
                let _ = handle.unsubscribe();
            }
            println!("Subscription group untracked: {:?}", key);
            true
        } else {
            false
        }
    }

    /// Unsubscribe all active subscriptions across all groups and clear the collection.
    pub fn unsubscribe_all(&mut self) {
        for (_key, group) in self.data.drain() {
            for handle in group {
                let _ = handle.unsubscribe();
            }
        }
    }

    /// Returns true if there is a non-empty subscription group for the key.
    pub fn contains(&self, key: SubKey) -> bool {
        self.data.get(&key).map_or(false, |group| !group.is_empty())
    }

    /// Total number of active subscriptions tracked across all groups.
    pub fn len(&self) -> usize {
        self.data.values().map(|group| group.len()).sum()
    }

    /// Returns the number of groups (keys) tracked.
    pub fn group_count(&self) -> usize {
        self.data.len()
    }

    /// Returns true if no subscriptions are tracked.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Drop for StdbSubscriptions {
    fn drop(&mut self) {
        // Ensure we don't leak active subscriptions on shutdown or world teardown.
        for (_key, group) in self.data.drain() {
            for handle in group {
                let _ = handle.unsubscribe();
            }
        }
    }
}
