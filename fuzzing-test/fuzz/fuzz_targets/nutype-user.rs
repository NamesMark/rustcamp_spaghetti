#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|string: &str| {
    //if let Ok(string) = std::str::from_utf8(data) {
    if string.chars().all(|c| c.is_ascii() && c.is_alphabetic()) {
        fuzzing_test::UserName::try_new(string).expect("Bad username");
    }
    //}
});
