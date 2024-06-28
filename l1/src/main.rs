fn main() {
    let a = 5;
    let boxed_a = Box::new(a);

    println!("{boxed_a}");
    {
        let boxed_a = Box::new(42);
        println!("{boxed_a}");
    }

    println!("{boxed_a}");

    let strange_vec = vec![5, 'a' as i32];
    println!("{strange_vec:?}");
}

fn increment(mut number: Box<i32>) -> Box<i32> {
    *number += 1;
    number
}
