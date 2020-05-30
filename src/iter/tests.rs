use crate::ByteSet;

#[test]
fn collect_full() {
    let set = ByteSet::full();

    let bytes: Vec<u8> = set.into_iter().collect();
    assert_eq!(bytes.len(), 256);

    for b in 0..=u8::max_value() {
        let i = b as usize;

        assert_eq!(bytes.get(i), Some(&b), "{:?} at {} is not {}", bytes, i, b);
    }
}

#[test]
fn collect_full_rev() {
    let set = ByteSet::full();

    let bytes: Vec<u8> = set.into_iter().rev().collect();
    assert_eq!(bytes.len(), 256);

    for b in 0..=u8::max_value() {
        // Iterating in reverse, so flip the index.
        let i = 255 - b as usize;

        assert_eq!(bytes.get(i), Some(&b), "{:?} at {} is not {}", bytes, i, b);
    }
}
