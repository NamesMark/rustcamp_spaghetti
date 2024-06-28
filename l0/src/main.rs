const NUM: usize = 253647;
const STR: &str = "delicious";

fn main() {
    let hello = "HEYA GUYS";
    let sum = sum(5, 6);
    println!("{hello}! Sum is {sum}");
    let num = NUM;
    let str = STR;
    println!("{num} is {str}");
}

fn sum(a: usize, b: usize) -> usize {
    a + b
}