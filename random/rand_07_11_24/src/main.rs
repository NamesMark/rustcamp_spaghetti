use rand::prelude::SliceRandom;

fn main() {
    let mut counts = vec![0;3];
    let mut rng = rand::thread_rng();
    let choices = [0, 1, 2];

    for _ in 0..2000 {
        let idx: usize = *choices.choose(&mut rng).unwrap();
        counts[idx] += 1;
    }

    println!("{:?}", counts);

    let a = std::borrow::Cow::from("string".to_string());
    println!("{}", std::any::type_name_of_val(&a));
}
