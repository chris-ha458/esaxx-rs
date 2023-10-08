//! Small wrapper around sentencepiece's esaxx suffix array C++ library.
//! Usage
//!
//! ```rust
//! #[cfg(feature="cpp")]
//! {
//! let string = "abracadabra";
//! let suffix = esaxx_rs::c_ver::suffix(string).unwrap();
//! let chars: Vec<_> = string.chars().collect();
//! let mut iter = suffix.iter();
//! assert_eq!(iter.next().unwrap(), (&chars[..4], 2)); // abra
//! assert_eq!(iter.next(), Some((&chars[..1], 5))); // a
//! assert_eq!(iter.next(), Some((&chars[1..4], 2))); // bra
//! assert_eq!(iter.next(), Some((&chars[2..4], 2))); // ra
//! assert_eq!(iter.next(), Some((&chars[..0], 11))); // ''
//! assert_eq!(iter.next(), None);
//! }
//! ```
//!
//! The previous version uses unsafe optimized c++ code.
//! There exists another implementation a bit slower (~2x slower) that uses
//! safe rust. It's a bit slower because it uses usize (mostly 64bit) instead of i32 (32bit).
//! But it does seems to fix a few OOB issues in the cpp version
//! (which never seemed to cause real problems in tests but still.)
//!
//! ```rust
//! let string = "abracadabra";
//! let suffix = esaxx_rs::suffix_rs(string).unwrap();
//! let chars: Vec<_> = string.chars().collect();
//! let mut iter = suffix.iter();
//! assert_eq!(iter.next().unwrap(), (&chars[..4], 2)); // abra
//! assert_eq!(iter.next(), Some((&chars[..1], 5))); // a
//! assert_eq!(iter.next(), Some((&chars[1..4], 2))); // bra
//! assert_eq!(iter.next(), Some((&chars[2..4], 2))); // ra
//! assert_eq!(iter.next(), Some((&chars[..0], 11))); // ''
//! assert_eq!(iter.next(), None);
//! ```

pub mod c_ver;
mod esa;
mod sais;
mod structs;
mod types;

use esa::esaxx_rs;
use structs::Suffix;
use types::SuffixError;

/// Creates the suffix array and provides an iterator over its items (Rust version)
/// See [suffix](fn.suffix.html)
///
/// Gives you an iterator over the suffixes of the input array and their count within
/// the input string.
/// ```rust
/// let string = "abracadabra";
/// let suffix = esaxx_rs::suffix_rs(string).unwrap();
/// let chars: Vec<_> = string.chars().collect();
/// let mut iter = suffix.iter();
/// assert_eq!(iter.next().unwrap(), (&chars[..4], 2)); // abra
/// assert_eq!(iter.next(), Some((&chars[..1], 5))); // a
/// assert_eq!(iter.next(), Some((&chars[1..4], 2))); // bra
/// assert_eq!(iter.next(), Some((&chars[2..4], 2))); // ra
/// assert_eq!(iter.next(), Some((&chars[..0], 11))); // ''
/// assert_eq!(iter.next(), None);
/// ```
pub fn suffix_rs(string: &str) -> Result<Suffix<usize>, SuffixError> {
    let chars: Vec<_> = string.chars().collect();
    let n: usize = chars.len();
    let u32_chars: Vec<u32> = chars.iter().map(|c| *c as u32).collect::<Vec<_>>();
    let mut suffix_array: Vec<usize> = vec![0; n];
    let mut left_array: Vec<usize> = vec![0; n];
    let mut right_array: Vec<usize> = vec![0; n];
    let mut depth_array: Vec<usize> = vec![0; n];
    let alphabet_size = 0x110000; // All UCS4 range.
    let node_num = esaxx_rs(
        &u32_chars,
        &mut suffix_array,
        &mut left_array,
        &mut right_array,
        &mut depth_array,
        alphabet_size,
    )?;
    Ok(Suffix {
        chars,
        suffix_array,
        left_array,
        right_array,
        depth_array,
        node_num,
    })
}

#[cfg(test)]
mod rs_tests {
    use super::*;

    #[test]
    fn test_esaxx_rs() {
        let string = "abracadabra".to_string();
        let chars: Vec<_> = string.chars().map(|c| c as u32).collect();
        let n = chars.len();
        let mut sa = vec![0; n];
        let mut l = vec![0; n];
        let mut r = vec![0; n];
        let mut d = vec![0; n];
        let alphabet_size = 0x110000; // All UCS4 range.

        let node_num = esaxx_rs(&chars, &mut sa, &mut l, &mut r, &mut d, alphabet_size).unwrap();
        println!("Node num {}", node_num);
        println!("sa {:?}", sa);
        println!("l {:?}", l);
        println!("r {:?}", r);
        println!("d {:?}", d);
        assert_eq!(node_num, 5);
        assert_eq!(sa, vec![10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
        assert_eq!(l, vec![1, 0, 5, 9, 0, 0, 3, 0, 0, 0, 2]);
        assert_eq!(r, vec![3, 5, 7, 11, 11, 1, 0, 1, 0, 0, 0]);
        assert_eq!(d, vec![4, 1, 3, 2, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_esaxx_rs_long() {
        let string = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.".to_string();
        let chars: Vec<_> = string.chars().map(|c| c as u32).collect();
        let n = chars.len();
        let mut sa = vec![0; n];
        let mut l = vec![0; n];
        let mut r = vec![0; n];
        let mut d = vec![0; n];
        let alphabet_size = 0x110000; // All UCS4 range.

        let node_num = esaxx_rs(&chars, &mut sa, &mut l, &mut r, &mut d, alphabet_size).unwrap();
        assert_eq!(chars.len(), 574);
        assert_eq!(node_num, 260);
        // assert_eq!(sa, vec![10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
        // assert_eq!(l, vec![1, 0, 5, 9, 0, 0, 3, 0, 0, 0, 2]);
        // assert_eq!(r, vec![3, 5, 7, 11, 11, 1, 0, 1, 0, 0, 0]);
        // assert_eq!(d, vec![4, 1, 3, 2, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_suffix_rs() {
        let suffix = suffix_rs("abracadabra").unwrap();
        assert_eq!(suffix.node_num, 5);
        assert_eq!(suffix.suffix_array, vec![10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
        assert_eq!(suffix.left_array, vec![1, 0, 5, 9, 0, 0, 3, 0, 0, 0, 2]);
        assert_eq!(suffix.right_array, vec![3, 5, 7, 11, 11, 1, 0, 1, 0, 0, 0]);
        assert_eq!(suffix.depth_array, vec![4, 1, 3, 2, 0, 0, 0, 0, 0, 0, 0]);

        let mut iter = suffix.iter();
        let chars: Vec<_> = "abracadabra".chars().collect();
        assert_eq!(iter.next(), Some((&chars[..4], 2))); // abra
        assert_eq!(iter.next(), Some((&chars[..1], 5))); // a
        assert_eq!(iter.next(), Some((&chars[1..4], 2))); // bra
        assert_eq!(iter.next(), Some((&chars[2..4], 2))); // ra
        assert_eq!(iter.next(), Some((&chars[..0], 11))); // ''
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_out_of_bounds_bug() {
        let string = "banana$band$$";
        suffix_rs(string).unwrap();
    }
}
