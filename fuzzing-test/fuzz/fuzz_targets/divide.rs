#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (i32, i32)| {
    fuzzing_test::divide(data.0, data.1);
});
