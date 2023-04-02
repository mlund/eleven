use alloc::{string::String, vec::Vec};
use ufmt_stdio::*;

pub struct Label {
    /// lb$ = label name
    name: String,
    /// ll$ = (post-processed line)
    pp_line: u16,
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
pub fn parse_label(
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
    (*labels).push(Label {
        name: String::from(&((*current_line)[1..])),
        pp_line: pp_line + 1,
    });
}
