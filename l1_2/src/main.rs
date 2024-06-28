fn main() {
    let string = "ýčec👉🥺👈 🦀 ö".to_string();
    //let string_slice = &string[1..6];
    //println!("{string_slice}");
    let string_two = string.chars().filter(|&s| s != '🦀').collect::<String>();
    println!("{string_two}");
}