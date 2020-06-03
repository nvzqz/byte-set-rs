use super::ByteSet;

#[test]
fn len() {
    assert_len!(ByteSet::new(), 0);
    assert_len!(ByteSet::full(), 256);
}

#[test]
fn insert() {
    let mut set = ByteSet::new();

    for byte in 0..=u8::max_value() {
        assert_not_contains!(set, byte);

        let copy = set;
        assert_contains!(copy.inserting(byte), byte);

        set.insert(byte);
        assert_contains!(set, byte);
    }

    assert_eq!(set.len(), 256);
}

#[test]
fn remove() {
    let mut set = ByteSet::full();

    for byte in 0..=u8::max_value() {
        assert_contains!(set, byte);

        let copy = set;
        assert_not_contains!(copy.removing(byte), byte);

        set.remove(byte);
        assert_not_contains!(set, byte);
    }

    assert_empty!(set);
}

#[test]
fn first() {
    macro_rules! assert_first_eq {
        ($set:expr, $first:expr) => {{
            let set = &$set;
            let first: Option<u8> = $first.into();
            assert_eq!(
                set.first(),
                first,
                "First byte in {} is not {:?}",
                set.fmt_binary(),
                first
            );
        }};
    }

    assert_first_eq!(ByteSet::new(), None);
    assert_eq!(ByteSet::new().pop_first(), None);

    let mut set = ByteSet::full();

    for byte in set.into_iter() {
        assert_first_eq!(set, byte);
        assert_eq!(set.pop_first(), Some(byte));
    }

    assert_first_eq!(set, None);
    assert_eq!(set.pop_first(), None);
}

#[test]
fn last() {
    macro_rules! assert_last_eq {
        ($set:expr, $last:expr) => {{
            let set = &$set;
            let last: Option<u8> = $last.into();
            assert_eq!(
                set.last(),
                last,
                "Last byte in {} is not {:?}",
                set.fmt_binary(),
                last
            );
        }};
    }

    assert_last_eq!(ByteSet::new(), None);
    assert_eq!(ByteSet::new().pop_last(), None);

    let mut set = ByteSet::full();

    for byte in set.into_iter().rev() {
        assert_last_eq!(set, byte);
        assert_eq!(set.pop_last(), Some(byte));
    }

    assert_last_eq!(set, None);
    assert_eq!(set.pop_last(), None);
}

#[test]
fn maybe_contains() {
    let mut set = ByteSet::new();

    for byte in 0..=u8::max_value() {
        assert!(!set.maybe_contains(byte));

        set.insert(byte);
        assert!(set.maybe_contains(byte));

        set.remove(byte);
        assert!(!set.maybe_contains(byte));
    }
}
