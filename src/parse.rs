use alloc::{string::String, string::ToString, vec::Vec};
use ufmt_stdio::*;

pub const WHITESPACE_CHARS: [u8; 4] = [32, 160, 29, 9]; // space, shift+space, right, tab

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
        name: current_line.into(),
        pp_line: pp_line + 1,
    });
}

pub fn trim_left<'a>(line: &'a str, trim_chars: &[u8]) -> &'a str {
    let mut i = 0;
    while i < line.len() && trim_chars.contains(&line.as_bytes()[i]) {
        i += 1;
    }
    &line[i..]
}

pub fn trim_right<'a>(line: &'a str, trim_chars: &[u8]) -> &'a str {
    let mut i = (line.len() - 1) as i16;
    while i >= 0 && trim_chars.contains(&line.as_bytes()[i as usize]) {
        i = i - 1;
    }
    &line[..((i + 1) as usize)]
}

pub fn trim_left_white_space<'a>(line: &'a str) -> &'a str {
    trim_left(line, &WHITESPACE_CHARS)
}

pub fn trim_right_white_space<'a>(line: &'a str) -> &'a str {
    trim_right(line, &WHITESPACE_CHARS)
}

// pub fn trim_line<'a>(line: &'a str) -> &'a str {
//     // 340
//     let trimmed = trim_left_white_space(&line);

//     single_quote_comment_trim(&mut current_line);

//     //560-580
//     if !current_line.is_empty() {
//         current_line = trim_right_white_space(&current_line).into();
//     }
// }

pub fn single_quote_comment_trim(current_line: &mut String) {
    //422
    if current_line.find('\'').is_none() || current_line.find('"').is_none() {
        return;
    }
    //423
    //424
    let mut quote_flag = false;
    let mut cut_tail_idx = None;
    //440
    for (in_line_idx, c) in current_line.chars().enumerate() {
        //let c = (*current_line).chars().nth(in_line_idx).unwrap();
        match c {
            '"' => quote_flag = !quote_flag,
            '\'' => {
                if !quote_flag {
                    cut_tail_idx = Some(in_line_idx);
                    break;
                }
            }
            _ => (),
        }
    }
    //540
    if cut_tail_idx.is_some() {
        *current_line = current_line[..cut_tail_idx.unwrap()].to_string();
    }
    //println!("'{}'", &(*current_line)[..]);
}
