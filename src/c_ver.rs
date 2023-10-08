use crate::structs::Suffix;
use crate::types::*;

#[cfg(feature = "cc")]
extern "C" {
    fn esaxx_int32(
        // This is char32
        T: *const u32,
        SA: *mut i32,
        L: *mut i32,
        R: *mut i32,
        D: *mut i32,
        n: u32,
        k: u32,
        nodeNum: &mut u32,
    ) -> i32;
}

#[cfg(feature = "cc")]
pub(crate) fn esaxx(
    chars: &[char],
    sa: &mut [i32],
    l: &mut [i32],
    r: &mut [i32],
    d: &mut [i32],
    alphabet_size: u32,
    node_num: &mut u32,
) -> Result<(), SuffixError> {
    let n = chars.len();
    if sa.len() != n || l.len() != n || r.len() != n || d.len() != n {
        return Err(SuffixError::InvalidLength);
    }
    unsafe {
        let err = esaxx_int32(
            chars.as_ptr() as *const u32,
            sa.as_mut_ptr(),
            l.as_mut_ptr(),
            r.as_mut_ptr(),
            d.as_mut_ptr(),
            n as u32,
            alphabet_size,
            node_num,
        );
        if err != 0 {
            return Err(SuffixError::Internal);
        }
    }
    Ok(())
}

/// Creates the suffix array and provides an iterator over its items (c++ unsafe version)
///
/// Gives you an iterator over the suffixes of the input array and their count within
/// the input string.
/// ```rust
/// let string = "abracadabra";
/// let suffix = esaxx_rs::c_ver::suffix(string).unwrap();
/// let chars: Vec<_> = string.chars().collect();
/// let mut iter = suffix.iter();
/// assert_eq!(iter.next().unwrap(), (&chars[..4], 2)); // abra
/// assert_eq!(iter.next(), Some((&chars[..1], 5))); // a
/// assert_eq!(iter.next(), Some((&chars[1..4], 2))); // bra
/// assert_eq!(iter.next(), Some((&chars[2..4], 2))); // ra
/// assert_eq!(iter.next(), Some((&chars[..0], 11))); // ''
/// assert_eq!(iter.next(), None);
/// ```
#[cfg(feature = "cpp")]
pub fn suffix(string: &str) -> Result<Suffix<i32>, SuffixError> {
    let chars: Vec<_> = string.chars().collect();
    let n = chars.len();
    let mut sa = vec![0; n];
    let mut l = vec![0; n];
    let mut r = vec![0; n];
    let mut d = vec![0; n];
    let mut node_num = 0;
    let alphabet_size = 0x110000; // All UCS4 range.
    esaxx(
        &chars,
        &mut sa,
        &mut l,
        &mut r,
        &mut d,
        alphabet_size,
        &mut node_num,
    )?;
    Ok(Suffix {
        chars,
        suffix_array: sa,
        left_array: l,
        right_array: r,
        depth_array: d,
        node_num: node_num as usize,
    })
}

#[cfg(test)]
#[cfg(feature = "cpp")]
mod cpp_tests {
    use super::*;

    #[test]
    fn test_esaxx() {
        let string = "abracadabra".to_string();
        let chars: Vec<_> = string.chars().collect();
        let n = chars.len();
        let mut sa = vec![0; n];
        let mut l = vec![0; n];
        let mut r = vec![0; n];
        let mut d = vec![0; n];
        let mut node_num = 0;
        let alphabet_size = 0x110000; // All UCS4 range.

        esaxx(
            &chars,
            &mut sa,
            &mut l,
            &mut r,
            &mut d,
            alphabet_size,
            &mut node_num,
        )
        .unwrap();
        assert_eq!(node_num, 5);
        assert_eq!(sa, vec![10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
        assert_eq!(l, vec![1, 0, 5, 9, 0, 0, 3, 0, 0, 0, 2]);
        assert_eq!(r, vec![3, 5, 7, 11, 11, 1, 0, 1, 0, 0, 0]);
        assert_eq!(d, vec![4, 1, 3, 2, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_esaxx_long() {
        let string = "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.".to_string();
        let chars: Vec<_> = string.chars().collect();
        let n = chars.len();
        let mut sa = vec![0; n];
        let mut l = vec![0; n];
        let mut r = vec![0; n];
        let mut d = vec![0; n];
        let mut node_num = 0;
        let alphabet_size = 0x110000; // All UCS4 range.

        esaxx(
            &chars,
            &mut sa,
            &mut l,
            &mut r,
            &mut d,
            alphabet_size,
            &mut node_num,
        )
        .unwrap();
        assert_eq!(chars.len(), 574);
        assert_eq!(node_num, 260);
        // assert_eq!(sa, vec![10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);
        // assert_eq!(l, vec![1, 0, 5, 9, 0, 0, 3, 0, 0, 0, 2]);
        // assert_eq!(r, vec![3, 5, 7, 11, 11, 1, 0, 1, 0, 0, 0]);
        // assert_eq!(d, vec![4, 1, 3, 2, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_suffix() {
        let suffix = suffix("abracadabra").unwrap();
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
}
