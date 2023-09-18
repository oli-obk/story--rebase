use color_eyre::eyre::bail;
use color_eyre::Result;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub struct SortedMap<K, V> {
    entries: Vec<V>,
    entry_by_key: HashMap<K, usize>,
}

impl<K, V> Default for SortedMap<K, V> {
    fn default() -> Self {
        Self {
            entries: Default::default(),
            entry_by_key: Default::default(),
        }
    }
}

impl<K: Debug, V: Debug> Debug for SortedMap<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K: Eq + PartialEq + Hash + Debug, V: Debug> SortedMap<K, V> {
    pub fn insert(&mut self, key: K, val: V) -> Result<()> {
        match self.entry_by_key.entry(key) {
            Entry::Occupied(o) => {
                bail!("{o:?}")
            }
            Entry::Vacant(v) => {
                v.insert(self.entries.len());
                self.entries.push(val);
                Ok(())
            }
        }
    }

    pub fn get_or_insert_default(&mut self, key: K) -> &mut V
    where
        V: Default,
    {
        let idx = match self.entry_by_key.entry(key) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(v) => {
                let idx = self.entries.len();
                v.insert(idx);
                self.entries.push(Default::default());
                idx
            }
        };
        &mut self.entries[idx]
    }

    pub fn get(&self, index: &K) -> Option<&V> {
        Some(&self.entries[*self.entry_by_key.get(index)?])
    }
}

impl<K, V> SortedMap<K, V> {
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.entries.iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.entries.iter().enumerate().map(|(i, v)| {
            (
                self.entry_by_key
                    .iter()
                    .find(|(_, &idx)| idx == i)
                    .unwrap()
                    .0,
                v,
            )
        })
    }
}
