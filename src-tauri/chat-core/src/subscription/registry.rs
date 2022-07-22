use super::mvec::{self, MVec, MVecIterMut};
use std::borrow::Borrow;
use std::collections::{hash_map::Entry, HashMap};
use std::hash::Hash;

/// A registry of channels
pub(crate) struct Registry<K, V>
where
    K: Eq + Hash,
{
    pub(super) map: HashMap<K, MVec<V>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ID(usize);

/// NewType to protect access to the registry ID.
#[derive(Debug)]
pub enum RegistrationEffect {
    NewName,
    ExistingName,
}

#[derive(Debug)]
pub enum UnRegistrationEffect {
    NameErased,
    NamePreserved,
}

impl<K, V> Registry<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: K, value: V) -> (ID, RegistrationEffect) {
        let entry = self.map.entry(name);
        let effect = match &entry {
            Entry::Vacant(_) => RegistrationEffect::NewName,
            Entry::Occupied(_) => RegistrationEffect::ExistingName,
        };
        let mvec = entry.or_default();
        let id = mvec.counter();
        mvec.push(value);
        (ID(id), effect)
    }

    pub fn unregister<Q: ?Sized>(&mut self, name: &Q, id: ID) -> Option<(V, UnRegistrationEffect)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let ID(id) = id;
        let mvec = self.map.get_mut(name)?;
        let removed = mvec.remove(id)?;

        let effect = if mvec.is_empty() {
            self.map.remove(name);
            UnRegistrationEffect::NameErased
        } else {
            UnRegistrationEffect::NamePreserved
        };
        Some((removed, effect))
    }

    pub fn get_iter_mut<'a, Q: ?Sized>(&'a mut self, name: &Q) -> Option<MVecIterMut<'a, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.map.get_mut(name).map(MVec::iter_mut)
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.map.keys()
    }
}
