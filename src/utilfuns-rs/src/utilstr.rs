use std::ffi::{CStr, CString, c_char};

const ASCII_WHITESPACE: &[u8] = b" \t\n\r";

fn ascii_fold_byte(value: u8) -> u8 {
    value.to_ascii_lowercase()
}

fn trim_ascii_whitespace(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|b| !ASCII_WHITESPACE.contains(b))
        .unwrap_or(bytes.len());

    let end = bytes
        .iter()
        .rposition(|b| !ASCII_WHITESPACE.contains(b))
        .map(|idx| idx + 1)
        .unwrap_or(start);

    &bytes[start..end]
}

fn compare_ascii_case_insensitive_ascii(left: &[u8], right: &[u8]) -> i32 {
    let limit = left.len().max(right.len());

    for index in 0..limit {
        let left_byte = left.get(index).copied().unwrap_or(0);
        let right_byte = right.get(index).copied().unwrap_or(0);

        if left_byte == 0 && right_byte == 0 {
            return 0;
        }

        let diff = (ascii_fold_byte(left_byte) as i32) - (ascii_fold_byte(right_byte) as i32);
        if diff != 0 {
            return diff;
        }

        if left_byte == 0 || right_byte == 0 {
            return 0;
        }
    }

    0
}

fn compare_ascii_case_insensitive_limited(left: &[u8], right: &[u8], limit: usize) -> i32 {
    if limit == 0 {
        return 0;
    }

    for index in 0..limit {
        let left_byte = left.get(index).copied().unwrap_or(0);
        let right_byte = right.get(index).copied().unwrap_or(0);

        if left_byte == 0 && right_byte == 0 {
            return 0;
        }

        let diff = (ascii_fold_byte(left_byte) as i32) - (ascii_fold_byte(right_byte) as i32);
        if diff != 0 {
            return diff;
        }

        if left_byte == 0 || right_byte == 0 {
            return 0;
        }
    }

    0
}

pub fn strstrip(input: &str) -> &str {
    let bytes = input.as_bytes();
    let trimmed = trim_ascii_whitespace(bytes);
    let start = trimmed.as_ptr() as usize - input.as_ptr() as usize;
    let end = start + trimmed.len();
    &input[start..end]
}

pub fn stristr(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }

    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();

    let haystack_lower: Vec<u8> = haystack_bytes.iter().map(|b| ascii_fold_byte(*b)).collect();
    let needle_lower: Vec<u8> = needle_bytes.iter().map(|b| ascii_fold_byte(*b)).collect();

    haystack_lower
        .windows(needle_lower.len())
        .position(|window| window == needle_lower.as_slice())
}

pub fn strnicmp(left: &str, right: &str, len: i32) -> i32 {
    if len <= 0 {
        return 0;
    }

    compare_ascii_case_insensitive_limited(left.as_bytes(), right.as_bytes(), len as usize)
}

pub fn stricmp(left: &str, right: &str) -> i32 {
    compare_ascii_case_insensitive_ascii(left.as_bytes(), right.as_bytes())
}

pub fn assure_valid_utf8<T: AsRef<[u8]>>(input: T) -> String {
    String::from_utf8_lossy(input.as_ref()).into_owned()
}

#[unsafe(no_mangle)]
pub extern "C" fn sword_strstrip(input: *mut c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let bytes = CStr::from_ptr(input).to_bytes();
        let trimmed = trim_ascii_whitespace(bytes);

        if trimmed.is_empty() {
            *input = 0;
            return input;
        }

        std::ptr::copy(trimmed.as_ptr(), input as *mut u8, trimmed.len());
        *input.add(trimmed.len()) = 0;
        input
    }
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

        let haystack_lower: Vec<u8> = haystack_bytes.iter().map(|b| ascii_fold_byte(*b)).collect();
        let needle_lower: Vec<u8> = needle_bytes.iter().map(|b| ascii_fold_byte(*b)).collect();

        match haystack_lower
            .windows(needle_lower.len())
            .position(|window| window == needle_lower.as_slice())
        {
            Some(offset) => haystack.add(offset),
            None => std::ptr::null(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sword_strnicmp(left: *const c_char, right: *const c_char, len: i32) -> i32 {
    if left.is_null() || right.is_null() || len <= 0 {
        return 0;
    }

    unsafe {
        let left_bytes = CStr::from_ptr(left).to_bytes();
        let right_bytes = CStr::from_ptr(right).to_bytes();
        compare_ascii_case_insensitive_limited(left_bytes, right_bytes, len as usize)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sword_stricmp(left: *const c_char, right: *const c_char) -> i32 {
    if left.is_null() || right.is_null() {
        return 0;
    }

    unsafe {
        let left_bytes = CStr::from_ptr(left).to_bytes();
        let right_bytes = CStr::from_ptr(right).to_bytes();
        compare_ascii_case_insensitive_ascii(left_bytes, right_bytes)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sword_assure_valid_utf8(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let bytes = CStr::from_ptr(input).to_bytes();
        let repaired = String::from_utf8_lossy(bytes).into_owned();
        CString::new(repaired)
            .unwrap_or_else(|_| CString::new("").unwrap())
            .into_raw()
    }
}

