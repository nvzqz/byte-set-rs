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

macro_rules! assert_len {
    ($set:expr, $len:expr) => {
        assert_eq!(
            $set.len(),
            $len,
            "{:?} does not have {} bytes",
            $set.0,
            $len
        );
    };
}

macro_rules! assert_empty {
    ($set:expr) => {
        assert!($set.is_empty(), "{:?} is not empty", $set.0);
    };
}
