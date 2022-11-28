use std::cmp;

const BYTE_LEN: usize = 256;

/// Searches for a pattern of bytes into a byte slice and returns the indices of all occurences.
///
/// This function implements the Boyer-Moore string search algorithm, based on the [C
/// implementation of Wikipedia][1].
///
/// [1]: https://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string-search_algorithm#C_implementation
pub fn grep(text: &[u8], pattern: &[u8]) -> Option<Vec<usize>> {
    let mut results: Vec<usize> = Vec::new();
    let patlen = pattern.len();
    let mut to_search = text;
    let mut is_first_result = true;
    let mut last_index: usize = 0;

    while let Some(i) = boyer_moore(to_search, pattern) {
        if is_first_result {
            last_index += i;
            is_first_result = false;
        } else {
            last_index += i + patlen;
        }
        results.push(last_index);
        to_search = &text[last_index + patlen..];
    }

    if results.is_empty() {
        return None;
    }
    Some(results)
}

//---------------------BOYER-MOORE ALGORITHM-------------------------
fn bad_character_table(pattern: &[u8]) -> [usize; BYTE_LEN] {
    let patlen = pattern.len();
    let mut table = [patlen; BYTE_LEN];
    for (i, val) in pattern.iter().enumerate() {
        table[*val as usize] = patlen - 1 - i;
    }

    table
}

macro_rules! is_prefix {
    ($word:ident, $position:expr) => {
        $word
            .iter()
            .zip($word.iter().skip($position))
            .all(|(current, next)| current == next)
    };
}

fn suffix_lenght(word: &[u8], position: usize) -> usize {
    let suffix = &word[..position + 1];
    let mut count = 0;
    for (a, b) in suffix.iter().rev().zip(word.iter().rev()) {
        if a == b {
            count += 1;
            continue;
        }
        return count;
    }

    count
}

fn good_suffix_table(pattern: &[u8]) -> Vec<usize> {
    let patlen = pattern.len();
    let mut table: Vec<usize> = vec![0; patlen];
    let mut last_prefix_index = 1;

    for (i, val) in table.iter_mut().enumerate().rev() {
        if is_prefix!(pattern, i + 1) {
            last_prefix_index = i + 1;
        }
        *val = last_prefix_index + patlen - 1 - i;
    }

    for i in 0..patlen - 1 {
        let slen = suffix_lenght(pattern, i);
        if pattern[i - slen] != pattern[patlen - 1 - slen] {
            table[patlen - 1 - slen] = patlen - 1 - i + slen;
        }
    }

    table
}

fn boyer_moore(text: &[u8], pattern: &[u8]) -> Option<usize> {
    let bad_table = bad_character_table(pattern);
    let good_table = good_suffix_table(pattern);
    let patlen = pattern.len();
    let textlen = text.len();

    if textlen == 0 {
        return None;
    }

    let mut i: usize = patlen;
    while i < textlen + 1 {
        let mut equal = 0;
        let to_compare = &text[..i];
        for (a, b) in to_compare.iter().rev().zip(pattern.iter().rev()) {
            if a == b {
                equal += 1;
                continue;
            }
            break;
        }
        if equal == patlen {
            return Some(i - patlen);
        }
        let shift: usize = cmp::max(
            bad_table[text[i - 1] as usize],
            good_table[patlen - equal - 1],
        );
        i += shift;
    }

    None
}
