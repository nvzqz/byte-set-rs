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
