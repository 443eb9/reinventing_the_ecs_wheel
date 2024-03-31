use std::slice::{Iter, IterMut};

#[derive(Default, Clone)]
pub struct SparseSet {
    dense: Vec<u32>,
    sparse: Vec<Option<u32>>,
}

impl SparseSet {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, data: u32) {
        self.dense.push(data);
        let data = data as usize;
        if self.sparse.len() < data {
            self.sparse.resize(data + 1, None);
        }
        self.sparse[data] = Some(self.dense.len() as u32 - 1);
    }

    pub fn find_mut(&mut self, data: u32) -> Option<&mut u32> {
        let data = data as usize;
        if data > self.sparse.len() {
            None
        } else {
            self.sparse[data].map(|i| &mut self.dense[i as usize])
        }
    }

    pub fn remove(&mut self, data: u32) {
        if let Some(i) = self.sparse[data as usize] {
            self.sparse[data as usize] = None;
            if self.dense.len() != 1 {
                let t = *self.dense.last().unwrap() as usize;
                self.sparse[t] = Some(i);
            }
            self.dense.swap_remove(i as usize);
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, u32> {
        self.dense.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, u32> {
        self.dense.iter_mut()
    }

    #[inline]
    pub fn iter_sparse(&self) -> Iter<'_, Option<u32>> {
        self.sparse.iter()
    }

    #[inline]
    pub fn iter_sparse_mut(&mut self) -> IterMut<'_, Option<u32>> {
        self.sparse.iter_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sparse_set() {
        let mut set = SparseSet::default();
        set.add(2);
        set.add(4);
        set.add(1);
        assert!(set.find_mut(3).is_none());
        let mut dense = vec![2, 4, 1];
        let sparse = vec![None, Some(2), Some(0), None, Some(1)];
        set.iter()
            .enumerate()
            .for_each(|(i, e)| assert_eq!(*e, dense[i]));
        set.iter_sparse()
            .enumerate()
            .for_each(|(i, e)| assert_eq!(*e, sparse[i]));

        dense
            .iter_mut()
            .for_each(|i| assert_eq!(set.find_mut(*i), Some(i)));
        dense.iter().for_each(|i| {
            set.remove(*i);
        });

        assert!(set.is_empty());
    }
}
