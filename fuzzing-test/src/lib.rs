use nutype::nutype;

pub fn compute(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(divident: i32, divisor: i32) -> i32 {
    divident / divisor
}

#[nutype(
    sanitize(trim),
    validate(
        //not_empty, 
        len_char_max = 20, 
        regex = r"^[a-z]*$"
    ),
    derive(Debug, PartialEq)
)]
pub struct UserName(String);
