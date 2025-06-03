use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub enum MapDiff {
    Unchanged(EntryUnchanged),
    EntryAdded(EntryAdded),
    EntryRemoved(EntryRemoved),
    ValueModified(ValueModified),
    KeyModified(KeyModified),
}
#[derive(Debug, Clone, PartialEq)]
pub struct EntryUnchanged {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntryAdded {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntryRemoved {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueModified {
    pub key: String,
    pub old_value: Value,
    pub new_value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyModified {
    pub old_key: String,
    pub new_key: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueDiff {
    Unchanged,
    Modified { old_value: Value, new_value: Value },
}

impl MapDiff {
    pub fn is_change(&self) -> bool {
        !matches!(self, MapDiff::Unchanged(_))
    }
}

fn map_diff(left: &HashMap<String, Value>, right: &HashMap<String, Value>) -> Vec<MapDiff> {
    // Collect all keys from both maps
    let all_keys: HashSet<String> = left.keys().chain(right.keys()).cloned().collect();

    let mut diffs: Vec<MapDiff> = Vec::new();
    let mut entries_added: Vec<EntryAdded> = Vec::new();
    let mut entries_removed: Vec<EntryRemoved> = Vec::new();

    // Iterate, comparing values for all collected keys
    for key in all_keys {
        match (left.get(&key), right.get(&key)) {
            (Some(left_value), Some(right_value)) => {
                diffs.push(match diff_map_values(left_value, right_value) {
                    ValueDiff::Unchanged => MapDiff::Unchanged(EntryUnchanged {
                        key: key.clone(),
                        value: left_value.clone(),
                    }),
                    ValueDiff::Modified {
                        old_value,
                        new_value,
                    } => MapDiff::ValueModified(ValueModified {
                        key: key.clone(),
                        old_value,
                        new_value,
                    }),
                });
            }
            (Some(left_value), None) => entries_removed.push(EntryRemoved {
                key: key,
                value: left_value.clone(),
            }),
            (None, Some(right_value)) => entries_added.push(EntryAdded {
                key: key,
                value: right_value.clone(),
            }),
            (None, None) => unreachable!(),
        }
    }

    // Iterate on entries_removed, to see if any entries_added have the same value,
    // in which case we treat this as a change to a key rather than a deletion and insertion
    for entry in entries_removed {
        // Try to match it to an entry_added
        let matched_position = entries_added
            .iter()
            .position(|added| entry.value == added.value);
        if let Some(position) = matched_position {
            let added_entry = entries_added.swap_remove(position);
            diffs.push(MapDiff::KeyModified(KeyModified {
                old_key: entry.key.clone(),
                new_key: added_entry.key.clone(),
                value: entry.value.clone(),
            }));
        } else {
            diffs.push(MapDiff::EntryRemoved(entry));
        }
    }

    for entry in entries_added {
        diffs.push(MapDiff::EntryAdded(entry));
    }

    diffs
}

fn diff_map_values(left: &Value, right: &Value) -> ValueDiff {
    // For now, just compare the two directly
    if left == right {
        ValueDiff::Unchanged
    } else {
        ValueDiff::Modified {
            old_value: left.clone(),
            new_value: right.clone(),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use serde_json::json;

    use super::*;

    #[test]
    fn test_entry_removed() {
        let map_1 = HashMap::from([
            ("key_1".into(), "value_1".into()),
            ("key_2".into(), "value_2".into()),
            ("key_3".into(), "value_3".into()),
            ("key_4".into(), "value_4".into()),
        ]);

        let map_2 = HashMap::from([
            ("key_1".into(), "value_1".into()),
            ("key_2".into(), "value_2".into()),
            ("key_4".into(), "value_4".into()),
        ]);

        let diffs = map_diff(&map_1, &map_2);
        let changes = diffs
            .into_iter()
            .filter(|d| d.is_change())
            .collect::<Vec<_>>();

        assert_eq!(
            changes,
            vec![MapDiff::EntryRemoved(EntryRemoved {
                key: "key_3".into(),
                value: "value_3".into(),
            })],
        );
    }

    #[test]
    fn test_entry_added() {
        let map_1 = HashMap::from([
            ("key_1".into(), "value_1".into()),
            ("key_3".into(), "value_3".into()),
            ("key_4".into(), "value_4".into()),
        ]);

        let map_2 = HashMap::from([
            ("key_1".into(), "value_1".into()),
            ("key_2".into(), "value_2".into()),
            ("key_3".into(), "value_3".into()),
            ("key_4".into(), "value_4".into()),
        ]);

        let diffs = map_diff(&map_1, &map_2);
        let changes = diffs
            .into_iter()
            .filter(|d| d.is_change())
            .collect::<Vec<_>>();

        assert_eq!(
            changes,
            vec![MapDiff::EntryAdded(EntryAdded {
                key: "key_2".into(),
                value: "value_2".into(),
            })]
        );
    }

    #[test]
    fn test_value_modified() {
        let map_1 = HashMap::from([
            ("key_1".into(), "value_1".into()),
            ("key_2".into(), "value_2".into()),
            ("key_3".into(), "value_3".into()),
            ("key_4".into(), "value_4".into()),
        ]);

        let map_2 = HashMap::from([
            ("key_1".into(), "value_1".into()),
            ("key_2".into(), "value_2".into()),
            ("key_3".into(), "value_3.0".into()),
            ("key_4".into(), "value_4".into()),
        ]);

        let diffs = map_diff(&map_1, &map_2);
        let changes = diffs
            .into_iter()
            .filter(|d| d.is_change())
            .collect::<Vec<_>>();

        assert_eq!(
            changes,
            vec![MapDiff::ValueModified(ValueModified {
                key: "key_3".into(),
                old_value: "value_3".into(),
                new_value: "value_3.0".into(),
            })]
        );
    }

    #[test]
    fn test_key_modified() {
        let map_1 = HashMap::from([
            ("key_1".into(), "value_1".into()),
            ("key_2".into(), "value_2".into()),
            ("key_3".into(), "value_3".into()),
            ("key_4".into(), "value_4".into()),
        ]);

        let map_2 = HashMap::from([
            ("key_1".into(), "value_1".into()),
            ("key_2".into(), "value_2".into()),
            ("key_3.0".into(), "value_3".into()),
            ("key_4".into(), "value_4".into()),
        ]);

        let diffs = map_diff(&map_1, &map_2);
        let changes = diffs
            .into_iter()
            .filter(|d| d.is_change())
            .collect::<Vec<_>>();

        assert_eq!(
            changes,
            vec![MapDiff::KeyModified(KeyModified {
                old_key: "key_3".into(),
                new_key: "key_3.0".into(),
                value: "value_3".into(),
            })]
        );
    }

    #[test]
    fn test_entry_added_and_removed() {
        let map_1 = HashMap::from([
            ("key_1".into(), "value_1".into()),
            ("key_2".into(), "value_2".into()),
            ("key_3".into(), "value_3".into()),
            ("key_4".into(), "value_4".into()),
        ]);

        let map_2 = HashMap::from([
            ("key_1".into(), "value_1".into()),
            ("key_2".into(), "value_2".into()),
            ("key_3".into(), "value_3".into()),
            ("key_5".into(), "value_5".into()),
        ]);

        let diffs = map_diff(&map_1, &map_2);
        let changes = diffs
            .into_iter()
            .filter(|d| d.is_change())
            .collect::<Vec<_>>();

        assert_eq!(
            changes,
            vec![
                MapDiff::EntryRemoved(EntryRemoved {
                    key: "key_4".into(),
                    value: "value_4".into(),
                }),
                MapDiff::EntryAdded(EntryAdded {
                    key: "key_5".into(),
                    value: "value_5".into(),
                }),
            ],
        );
    }
}
