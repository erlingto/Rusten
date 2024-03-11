use std::hash::Hash;

#[derive(Clone, Debug)]
pub struct Dict<E: Eq + PartialEq, T> {
    map: std::collections::HashMap<E, T>,
}

impl<E: Eq + PartialEq + Hash, T> Dict<E, T> {
    pub fn insert(&mut self, discriminant: E, value: T) {
        self.map.insert(discriminant, value);
    }
}

impl<E: Eq + PartialEq + Hash, T> Default for Dict<E, T> {
    fn default() -> Self {
        Dict {
            map: std::collections::HashMap::new(),
        }
    }
}

impl<E: Eq + PartialEq + Hash, T> Dict<E, T> {
    pub fn new() -> Self {
        Dict {
            map: std::collections::HashMap::new(),
        }
    }
}

impl<E: Eq + PartialEq + Hash, T> std::ops::Index<E> for Dict<E, T> {
    type Output = T;
    fn index(&self, index: E) -> &Self::Output {
        &self.map[&index]
    }
}

impl<E: Eq + PartialEq + Hash, T> std::ops::IndexMut<E> for Dict<E, T> {
    fn index_mut(&mut self, index: E) -> &mut Self::Output {
        self.map.get_mut(&index).unwrap()
    }
}

impl<E: Eq + PartialEq + Hash, T> std::iter::IntoIterator for Dict<E, T> {
    type Item = (E, T);
    type IntoIter = std::collections::hash_map::IntoIter<E, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<E: Eq + PartialEq + Hash, T> std::iter::FromIterator<(E, T)> for Dict<E, T> {
    fn from_iter<I: IntoIterator<Item = (E, T)>>(iter: I) -> Self {
        let mut map = std::collections::HashMap::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        Dict { map }
    }
}

impl<E: Eq + PartialEq + Hash, T> Dict<E, T> {
    pub fn get(&self, key: E) -> Option<&T> {
        self.map.get(&key)
    }
}
