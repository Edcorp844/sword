use sword_utilfuns::{
    assure_valid_utf8, stricmp, stristr, strnicmp, strstrip,
    sword_stristr, sword_strstrip,
};

#[test]
fn trims_whitespace() {
    assert_eq!(strstrip("  hello world \n"), "hello world");
    assert_eq!(strstrip("\t\r\n"), "");
}

#[test]
fn ignores_case_when_searching() {
    assert_eq!(stristr("Hello World", "world"), Some(6));
    assert_eq!(stristr("Hello World", "HELLO"), Some(0));
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
fn c_abi_compatibility() {
    let mut buffer = b"  hello world\0".to_vec();
    let rust_ptr = buffer.as_mut_ptr() as *mut std::ffi::c_char;
    let out = sword_strstrip(rust_ptr);
    let actual = unsafe { std::ffi::CStr::from_ptr(out) }.to_str().unwrap();
    assert_eq!(actual, "hello world");

    let haystack = std::ffi::CString::new("Hello World").unwrap();
    let needle = std::ffi::CString::new("world").unwrap();
    let result = sword_stristr(haystack.as_ptr(), needle.as_ptr());
    let actual = unsafe { std::ffi::CStr::from_ptr(result) }.to_str().unwrap();
    assert_eq!(actual, "World");
}
