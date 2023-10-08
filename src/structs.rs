use std::convert::TryInto;

pub struct SuffixIterator<'a, T> {
    pub(super) i: usize,
    pub(super) suffix: &'a Suffix<T>,
}

pub struct Suffix<T> {
    pub(super) chars: Vec<char>,
    pub(super) suffix_array: Vec<T>,
    pub(super) left_array: Vec<T>,
    pub(super) right_array: Vec<T>,
    pub(super) depth_array: Vec<T>,
    pub(super) node_num: usize,
}

impl<T> Suffix<T> {
    pub fn iter(&self) -> SuffixIterator<'_, T> {
        SuffixIterator { i: 0, suffix: self }
    }
}

impl<'a> Iterator for SuffixIterator<'a, i32> {
    type Item = (&'a [char], u32);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.i;
        if index == self.suffix.node_num {
            None
        } else {
            let left: usize = self.suffix.left_array[index].try_into().ok()?;
            let offset: usize = self.suffix.suffix_array[left].try_into().ok()?;
            let len: usize = self.suffix.depth_array[index].try_into().ok()?;
            let freq: u32 = (self.suffix.right_array[index] - self.suffix.left_array[index])
                .try_into()
                .ok()?;
            self.i += 1;
            Some((&self.suffix.chars[offset..offset + len], freq))
        }
    }
}

impl<'a> Iterator for SuffixIterator<'a, usize> {
    type Item = (&'a [char], u32);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.i;
        if index == self.suffix.node_num {
            None
        } else {
            let left: usize = self.suffix.left_array[index];
            let offset: usize = self.suffix.suffix_array[left];
            let len: usize = self.suffix.depth_array[index];
            let freq: u32 = (self.suffix.right_array[index] - self.suffix.left_array[index])
                .try_into()
                .unwrap();
            self.i += 1;
            Some((&self.suffix.chars[offset..offset + len], freq))
        }
    }
}
