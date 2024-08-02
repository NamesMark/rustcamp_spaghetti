#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: i32| {
    fuzzing_test::compute(data, data);
});
