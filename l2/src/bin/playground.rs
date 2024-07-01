struct Birb(); // ZST = 0 bytes
struct Hooman(i32);
struct Gorilla(i32, i16);
enum ExtendedOption {
    Option1,
    Option2(i32),
}

fn main() {
    let extended_option = ExtendedOption::Option2(5);
    println!("{}", std::mem::size_of_val(&extended_option));

    // let mut maybe_hooman = Some(Hooman(5));
    // maybe_hooman = None;
    // println!("{}", std::mem::size_of_val(&maybe_hooman));


    // println!("{}", std::mem::size_of::<Birb>());
    // println!("{}", std::mem::size_of::<Option<Hooman>>());
    // println!("{}", std::mem::size_of::<Option<Gorilla>>());
}