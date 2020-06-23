use crate::ByteSet;

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
fn from_open_ranges() {
    for byte in 0..=u8::max_value() {
        let range_to = ..byte;
        let range_from = byte..;
        let range_to_i = ..=byte;
        let set_to = ByteSet::from_range_to(range_to.clone());
        let set_from = ByteSet::from_range_from(range_from.clone());
        let set_to_i = ByteSet::from_range_to_inclusive(range_to_i.clone());
        for b in 0..=u8::max_value() {
            assert_eq!(range_to.contains(&b), set_to.contains(b));
            assert_eq!(range_from.contains(&b), set_from.contains(b));
            assert_eq!(range_to_i.contains(&b), set_to_i.contains(b));
        }
    }
}

#[test]
fn from_closed_ranges() {
    for start in 0..=u8::max_value() {
        for end in start..=u8::max_value() {
            let range = start..end;
            let range_i = start..=end;
            let set = ByteSet::from_range(range.clone());
            let set_i = ByteSet::from_range_inclusive(range_i.clone());
            for b in 0..=u8::max_value() {
                assert_eq!(range.contains(&b), set.contains(b));
                assert_eq!(range_i.contains(&b), set_i.contains(b));
            }
        }
    }
}
