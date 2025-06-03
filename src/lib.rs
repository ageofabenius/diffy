pub mod diff;
pub mod file_loading;

// use similar::TextDiff;

// fn diff_text(left: &str, right: &str) {
//     let diff = TextDiff::configure().diff_chars(left, right);
//     dbg!(diff.ops());
//     for change in diff.ops() {
//         dbg!(change);
//         for foo in diff.iter_inline_changes(change) {
//             if foo.tag() == similar::ChangeTag::Equal {
//                 continue;
//             }
//             dbg!(foo);
//         }
//     }
//     // for change in diff.iter_all_changes() {
//     //     match change.tag() {
//     //         similar::ChangeTag::Delete => println!("-{}", change),
//     //         similar::ChangeTag::Insert => println!("+{}", change),
//     //         similar::ChangeTag::Equal => {}
//     //     };
//     // }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn foo() {
//         // Load sample.json
//         let sample_str = std::fs::read_to_string("test_data/sample_1/sample.json")
//             .expect("Failed to read sample.json");
//         let sample_diff_1_str = std::fs::read_to_string("test_data/sample_1/sample_diff_1.json")
//             .expect("Failed to read sample.json");

//         let diff = diff_text(&sample_str, &sample_diff_1_str);
//     }
// }
