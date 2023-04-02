use alloc::{string::String, vec::Vec};
use ufmt_stdio::*;

pub struct Label {
    /// lb$ = label name
    pub name: String,
    /// ll$ = (post-processed line)
    pub pp_line: u16,
}

/// Parse a single label and add it to the `labels` vector
///
/// ## Note
///
///   Original source code: 1500
///
/// ## Todo
///
/// return the label and let the caller add to `labels`.
pub fn add_label(
    verbose: bool,
    current_line: &str,
    pp_line: u16,
    delete_line_flag: &mut bool,
    labels: &mut Vec<Label>,
) {
    if verbose {
        println!("label {} at pp_line {}", current_line, pp_line);
    }
    *delete_line_flag = true;
    labels.push(Label {
        name: current_line[1..].into(),
        pp_line: pp_line + 1,
    });
}
