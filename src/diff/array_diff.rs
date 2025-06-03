// use similar::TextDiff;

// fn array_diff(left: &[String], right: &[String]) {
//     dbg!(left, right);
//     let left_strs: Vec<&str> = left.iter().map(String::as_str).collect();
//     let right_strs: Vec<&str> = right.iter().map(String::as_str).collect();
//     let diff = TextDiff::from_slices(&left_strs, &right_strs);
//     for change in diff.iter_all_changes() {
//        dbg!(change);
//     }
//     todo!()
// }

// #[cfg(test)]
// mod tests {

//     use std::vec;

//     use super::*;

//     #[test]
//     fn test_element_removed() {
//         let array_1 = vec![
//             "value_1".to_string(),
//             "value_2".to_string(),
//             "value_3".to_string(),
//             "value_4".to_string(),
//         ];

//         let array_2 = vec![
//             "value_1".to_string(),
//             "value_2".to_string(),
//             "value_4".to_string(),
//         ];

//         let diffs = array_diff(&array_1, &array_2);
//     }
// }
