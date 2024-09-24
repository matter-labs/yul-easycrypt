//!
//! Utility functions related to iterators.
//!

use std::collections::HashMap;
use std::hash::Hash;

/// Groups the items from a vector into a `HashMap` according to a key selector function.
///
/// # Arguments
///
/// * `items` - A vector of items to be grouped.
/// * `key_selector` - A function that takes a reference to an item and returns a key for grouping.
///
/// # Returns
///
/// A `HashMap` where the keys are determined by the `key_selector` function, and the values are
/// vectors of items that correspond to each key.
///
/// # Examples
///
/// ```
///     let items = vec!["apple", "banana", "apricot", "cherry", "blueberry"];
///
///     // Group by the first character of each string
/// let grouped = group_by(&items, |item| item.chars().next().unwrap());
///
/// assert_eq!(grouped.get(&'a'), Some(&vec!["apple", "apricot"]));
/// assert_eq!(grouped.get(&'b'), Some(&vec!["banana", "blueberry"]));
/// assert_eq!(grouped.get(&'c'), Some(&vec!["cherry"]));
/// ```
pub fn group_by<T, K, F>(items: &Vec<T>, key_selector: F) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut grouped_map: HashMap<K, Vec<T>> = HashMap::new();

    for item in items {
        let key = key_selector(&item);
        grouped_map
            .entry(key)
            .or_insert_with(Vec::new)
            .push(item.clone());
    }

    grouped_map
}

#[cfg(test)]
mod tests {
    use crate::util::iter::group_by;

    #[test]
    fn test_groupby() {
        let items = vec!["apple", "banana", "apricot", "cherry", "blueberry"];
        let grouped = group_by(&items, |item| item.chars().next().unwrap());

        assert_eq!(grouped.get(&'a'), Some(&vec!["apple", "apricot"]));
        assert_eq!(grouped.get(&'b'), Some(&vec!["banana", "blueberry"]));
        assert_eq!(grouped.get(&'c'), Some(&vec!["cherry"]));
    }
}
