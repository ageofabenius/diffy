// use similar::TextDiff;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum MapDiff {
    Unchanged {
        key: String,
        value: String,
    },
    EntryAdded {
        key: String,
        value: String,
    },
    EntryRemoved {
        key: String,
        value: String,
    },
    ValueModified {
        key: String,
        old_value: String,
        new_value: String,
    },
    KeyModified {
        old_key: String,
        new_key: String,
        value: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueDiff {
    Unchanged,
    Modified {
        old_value: String,
        new_value: String,
    },
}

fn map_diff(left: &HashMap<String, String>, right: &HashMap<String, String>) -> Vec<MapDiff> {
    let mut diffs: Vec<MapDiff> = Vec::new();

    let all_keys: HashSet<String> = left.keys().chain(right.keys()).cloned().collect();

    for key in all_keys {
        match (left.get(&key), right.get(&key)) {
            (Some(left_value), Some(right_value)) => {
                diffs.push(match diff_map_values(left_value, right_value) {
                    ValueDiff::Unchanged => MapDiff::Unchanged {
                        key: key.clone(),
                        value: left_value.clone(),
                    },
                    ValueDiff::Modified {
                        old_value,
                        new_value,
                    } => MapDiff::ValueModified {
                        key: key.clone(),
                        old_value,
                        new_value,
                    },
                });
            }
            (Some(left_value), None) => diffs.push(MapDiff::EntryRemoved {
                key: key,
                value: left_value.clone(),
            }),
            (None, Some(right_value)) => diffs.push(MapDiff::EntryAdded {
                key: key,
                value: right_value.clone(),
            }),
            (None, None) => unreachable!(),
        }
    }

    diffs
}

fn diff_map_values(left: &str, right: &str) -> ValueDiff {
    // let diff = TextDiff::configure().diff_chars(left, right);
    // let mut has_changes = false;
    // for changes in diff.grouped_ops(1) {
    //     has_changes = true;
    //     dbg!(changes);
    // }

    // if !has_changes {
    //     dbg!(format!(
    //         "No changes found between '{}' and '{}'",
    //         left, right
    //     ));
    // }

    // For now, just compare the two directly
    if left == right {
        ValueDiff::Unchanged
    } else {
        ValueDiff::Modified {
            old_value: left.to_string(),
            new_value: right.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn test_entry_removed() {
        let map_1 = HashMap::from([
            ("key_1".to_string(), "value_1".to_string()),
            ("key_2".to_string(), "value_2".to_string()),
            ("key_3".to_string(), "value_3".to_string()),
            ("key_4".to_string(), "value_4".to_string()),
        ]);

        let map_2 = HashMap::from([
            ("key_1".to_string(), "value_1".to_string()),
            ("key_2".to_string(), "value_2".to_string()),
            ("key_4".to_string(), "value_4".to_string()),
        ]);

        let diffs = map_diff(&map_1, &map_2);

        assert_eq!(diffs.len(), 1);
        assert_eq!(
            diffs,
            vec![MapDiff::EntryRemoved {
                key: "key_3".to_string(),
                value: "value_3".to_string(),
            }]
        );
    }

    #[test]
    fn test_entry_added() {
        let map_1 = HashMap::from([
            ("key_1".to_string(), "value_1".to_string()),
            ("key_3".to_string(), "value_3".to_string()),
            ("key_4".to_string(), "value_4".to_string()),
        ]);

        let map_2 = HashMap::from([
            ("key_1".to_string(), "value_1".to_string()),
            ("key_2".to_string(), "value_2".to_string()),
            ("key_3".to_string(), "value_3".to_string()),
            ("key_4".to_string(), "value_4".to_string()),
        ]);

        let diffs = map_diff(&map_1, &map_2);

        assert_eq!(diffs.len(), 1);
        assert_eq!(
            diffs,
            vec![MapDiff::EntryAdded {
                key: "key_2".to_string(),
                value: "value_2".to_string(),
            }]
        );
    }

    #[test]
    fn test_value_changed() {
        let map_1 = HashMap::from([
            ("key_1".to_string(), "value_1".to_string()),
            ("key_2".to_string(), "value_2".to_string()),
            ("key_3".to_string(), "value_3".to_string()),
            ("key_4".to_string(), "value_4".to_string()),
        ]);

        let map_2 = HashMap::from([
            ("key_1".to_string(), "value_1".to_string()),
            ("key_2".to_string(), "value_2".to_string()),
            ("key_3".to_string(), "value_3.0".to_string()),
            ("key_4".to_string(), "value_4".to_string()),
        ]);

        let diffs = map_diff(&map_1, &map_2);
        dbg!(&diffs);
    }
}
