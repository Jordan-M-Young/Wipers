use std::collections::HashSet;

pub fn lock_array_to_set(lock_array: Vec<String>) -> HashSet<String> {
    let mut lock_set: HashSet<String> = HashSet::new();

    for lock_file in lock_array {
        lock_set.insert(lock_file);
    }

    lock_set
}

#[cfg(test)]

mod tests {
    use std::collections::HashSet;

    use super::lock_array_to_set;

    #[test]
    fn test_lock_array_to_set() {
        let lock_array = vec!["./test-inputs/functions.py".to_string()];

        let expected_set = HashSet::from(["./test-inputs/functions.py".to_string()]);

        let actual_set = lock_array_to_set(lock_array);

        assert_eq!(expected_set, actual_set)
    }
}
