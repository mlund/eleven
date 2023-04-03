use alloc::{string::String, vec::Vec};
use ufmt_stdio::*;

const WHITESPACE_CHARS: [u8; 4] = [32, 160, 29, 9]; // space, shift+space, right, tab

/// Add brief description
///
/// Longer description...
pub struct Label {
    /// Label name (`lb`)
    pub name: String,
    /// Post-processed line number (`ll`)
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
        name: current_line.into(),
        pp_line: pp_line + 1,
    });
}

/// Remove matching `trim_chars` from beginning
pub fn trim_left<'a>(line: &'a str, trim_chars: &[u8]) -> &'a str {
    let mut i = 0;
    while i < line.len() && trim_chars.contains(&line.as_bytes()[i]) {
        i += 1;
    }
    &line[i..]
}

/// Remove trailing chars matching `trim_chars`
pub fn trim_right<'a>(line: &'a str, trim_chars: &[u8]) -> &'a str {
    let mut i = (line.len() - 1) as i16;
    while i >= 0 && trim_chars.contains(&line.as_bytes()[i as usize]) {
        i = i - 1;
    }
    &line[..((i + 1) as usize)]
}

/// Trim single quote comment
pub fn trim_single_quote_comment<'a>(line: &'a str) -> &'a str {
    //422
    if line.find('\'').is_none() || line.find('"').is_none() {
        return line;
    }
    //423
    //424
    let mut quote_flag = false;
    let mut cut_tail_idx = None;
    //440
    for (index, letter) in line.chars().enumerate() {
        //let c = (*current_line).chars().nth(in_line_idx).unwrap();
        match letter {
            '"' => quote_flag = !quote_flag,
            '\'' => {
                if !quote_flag {
                    cut_tail_idx = Some(index);
                    break;
                }
            }
            _ => (),
        }
    }
    //540
    return if cut_tail_idx.is_some() {
        &line[..cut_tail_idx.unwrap()]
    } else {
        line
    };
}

/// Combined trim that removes head and trailing white space, as well as comments
///
/// In BASIC: Line 340 and 560-580
pub fn trim_line<'a>(line: &'a str) -> &'a str {
    let mut trimmed = trim_left(&line, &WHITESPACE_CHARS);
    trimmed = trim_single_quote_comment(&trimmed);
    // @todo is this check needed?
    if !trimmed.is_empty() {
        trimmed = trim_right(&trimmed, &WHITESPACE_CHARS);
    }
    trimmed
}
