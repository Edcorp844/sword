use std::ffi::{CStr, c_char};

fn ascii_casefold(value: u8) -> u8 {
    value.to_ascii_lowercase()
}

pub fn strstrip(input: &str) -> &str {
    input.trim_matches(|ch: char| ch.is_whitespace())
}

#[unsafe(no_mangle)]
pub extern "C" fn sword_strstrip(input: *mut c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let bytes = CStr::from_ptr(input).to_bytes();
        let trimmed: Vec<u8> = bytes
            .iter()
            .copied()
            .skip_while(|byte| byte.is_ascii_whitespace())
            .collect::<Vec<_>>();

        let trimmed = trimmed
            .into_iter()
            .rev()
            .skip_while(|byte| byte.is_ascii_whitespace())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>();

        let len = trimmed.len();
        if len == 0 {
            *input = 0;
            return input;
        }

        std::ptr::copy_nonoverlapping(trimmed.as_ptr(), input as *mut u8, len);
        *input.add(len) = 0;
        input
    }
}

pub fn stristr(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }

    let haystack = haystack.to_ascii_lowercase();
    let needle = needle.to_ascii_lowercase();
    haystack.find(&needle)
}

#[unsafe(no_mangle)]
pub extern "C" fn sword_stristr(haystack: *const c_char, needle: *const c_char) -> *const c_char {
    if haystack.is_null() || needle.is_null() {
        return std::ptr::null();
    }

    unsafe {
        let haystack_bytes = CStr::from_ptr(haystack).to_bytes();
        let needle_bytes = CStr::from_ptr(needle).to_bytes();

        if needle_bytes.is_empty() {
            return haystack;
        }

        let haystack_lower: Vec<u8> = haystack_bytes.iter().map(|b| b.to_ascii_lowercase()).collect();
        let needle_lower: Vec<u8> = needle_bytes.iter().map(|b| b.to_ascii_lowercase()).collect();

        if needle_lower.len() > haystack_lower.len() {
            return std::ptr::null();
        }

        let position = haystack_lower
            .windows(needle_lower.len())
            .position(|window| window == needle_lower.as_slice())
            .unwrap_or(usize::MAX);

        if position == usize::MAX {
            return std::ptr::null();
        }

        haystack.add(position)
    }
}

pub fn strnicmp(left: &str, right: &str, len: i32) -> i32 {
    if len <= 0 {
        return 0;
    }

    let max_len = len as usize;
    for index in 0..max_len {
        let left_byte = left.as_bytes().get(index).copied().unwrap_or(0);
        let right_byte = right.as_bytes().get(index).copied().unwrap_or(0);

        if left_byte == 0 && right_byte == 0 {
            return 0;
        }

        let diff = (ascii_casefold(left_byte) as i32) - (ascii_casefold(right_byte) as i32);
        if diff != 0 {
            return diff;
        }

        if left_byte == 0 || right_byte == 0 {
            return 0;
        }
    }

    0
}

pub fn stricmp(left: &str, right: &str) -> i32 {
    let max_len = left.len().max(right.len());

    for index in 0..max_len {
        let left_byte = left.as_bytes().get(index).copied().unwrap_or(0);
        let right_byte = right.as_bytes().get(index).copied().unwrap_or(0);

        if left_byte == 0 && right_byte == 0 {
            return 0;
        }

        let diff = (ascii_casefold(left_byte) as i32) - (ascii_casefold(right_byte) as i32);
        if diff != 0 {
            return diff;
        }

        if left_byte == 0 || right_byte == 0 {
            return 0;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn sword_strnicmp(left: *const c_char, right: *const c_char, len: i32) -> i32 {
    if left.is_null() || right.is_null() || len <= 0 {
        return 0;
    }

    unsafe {
        let left_bytes = CStr::from_ptr(left).to_bytes();
        let right_bytes = CStr::from_ptr(right).to_bytes();
        let max_len = len as usize;

        for index in 0..max_len {
            let left_byte = left_bytes.get(index).copied().unwrap_or(0);
            let right_byte = right_bytes.get(index).copied().unwrap_or(0);

            if left_byte == 0 && right_byte == 0 {
                return 0;
            }

            let diff = (ascii_casefold(left_byte) as i32) - (ascii_casefold(right_byte) as i32);
            if diff != 0 {
                return diff;
            }

            if left_byte == 0 || right_byte == 0 {
                return 0;
            }
        }
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn sword_stricmp(left: *const c_char, right: *const c_char) -> i32 {
    if left.is_null() || right.is_null() {
        return 0;
    }

    unsafe {
        let left_bytes = CStr::from_ptr(left).to_bytes();
        let right_bytes = CStr::from_ptr(right).to_bytes();
        let max_len = left_bytes.len().max(right_bytes.len());

        for index in 0..max_len {
            let left_byte = left_bytes.get(index).copied().unwrap_or(0);
            let right_byte = right_bytes.get(index).copied().unwrap_or(0);

            if left_byte == 0 && right_byte == 0 {
                return 0;
            }

            let diff = (ascii_casefold(left_byte) as i32) - (ascii_casefold(right_byte) as i32);
            if diff != 0 {
                return diff;
            }

            if left_byte == 0 || right_byte == 0 {
                return 0;
            }
        }
    }

    0
}

pub fn assure_valid_utf8<T: AsRef<[u8]>>(input: T) -> String {
    String::from_utf8_lossy(input.as_ref()).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_whitespace() {
        assert_eq!(strstrip("  hello world \n"), "hello world");
    }

    #[test]
    fn ignores_case_when_searching() {
        assert_eq!(stristr("Hello World", "world"), Some(6));
    }

    #[test]
    fn compares_case_insensitive_prefix() {
        assert_eq!(strnicmp("AbC", "abc", 3), 0);
        assert!(strnicmp("AbC", "abd", 3) < 0);
    }

    #[test]
    fn compares_case_insensitive_strings() {
        assert_eq!(stricmp("Luke", "luke"), 0);
        assert!(stricmp("A", "b") < 0);
    }

    #[test]
    fn repairs_invalid_utf8() {
        let repaired = assure_valid_utf8(b"hi\xFFthere");
        assert_eq!(repaired, "hi\u{FFFD}there");
    }

    #[test]
    fn exported_c_abi_matches_legacy_behavior() {
        unsafe {
            let mut buf = *b"  hello world\0";
            let out = sword_strstrip(buf.as_mut_ptr() as *mut i8);
            let out = std::ffi::CStr::from_ptr(out).to_str().unwrap();
            assert_eq!(out, "hello world");

            let result = sword_stristr(c"Hello World".as_ptr(), c"world".as_ptr());
            let result = std::ffi::CStr::from_ptr(result).to_str().unwrap();
            assert_eq!(result, "World");
        }
    }
}
