use super::ByteSet;

// These assertions are macros in order to keep file/line/column info.

macro_rules! assert_not_contains {
    ($set:expr, $byte:expr) => {
        assert!(!$set.contains($byte), "{:?} contains {}", $set.0, $byte);
    };
}

macro_rules! assert_contains {
    ($set:expr, $byte:expr) => {
        assert!(
            $set.contains($byte),
            "{:?} does not contain {}",
            $set.0,
            $byte
        );
    };
}

macro_rules! assert_empty {
    ($set:expr) => {
        assert!($set.is_empty(), "{:?} is not empty", $set.0);
    };
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
