use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct dict<E: Eq + PartialEq, T> {
    map: std::collections::HashMap<E, T>,
}

impl<E: Eq + PartialEq + Hash, T> dict<E, T> {
    pub fn insert(&mut self, discriminant: E, value: T) {
        self.map.insert(discriminant, value);
    }
}

impl<E: Eq + PartialEq + Hash, T> Default for dict<E, T> {
    fn default() -> Self {
        dict {
            map: std::collections::HashMap::new(),
        }
    }
}

impl<E: Eq + PartialEq + Hash, T> dict<E, T> {
    pub fn new() -> Self {
        dict {
            map: std::collections::HashMap::new(),
        }
    }
}

impl<E: Eq + PartialEq + Hash, T> std::ops::Index<E> for dict<E, T> {
    type Output = T;
    fn index(&self, index: E) -> &Self::Output {
        &self.map[&index]
    }
}

impl<E: Eq + PartialEq + Hash, T> std::ops::IndexMut<E> for dict<E, T> {
    fn index_mut(&mut self, index: E) -> &mut Self::Output {
        self.map.get_mut(&index).unwrap()
    }
}

impl<E: Eq + PartialEq + Hash, T> std::iter::IntoIterator for dict<E, T> {
    type Item = (E, T);
    type IntoIter = std::collections::hash_map::IntoIter<E, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<E: Eq + PartialEq + Hash, T> std::iter::FromIterator<(E, T)> for dict<E, T> {
    fn from_iter<I: IntoIterator<Item = (E, T)>>(iter: I) -> Self {
        let mut map = std::collections::HashMap::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        dict { map }
    }
}

impl<E: Eq + PartialEq + Hash, T> dict<E, T> {
    pub fn get(&self, key: E) -> Option<&T> {
        self.map.get(&key)
    }
}
