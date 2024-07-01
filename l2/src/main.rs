use std::ops::Add;

struct Ball {
    diameter: i32,
}
struct Sphere {
    diameter: i32,
}

fn main() {
    let mut ball = Ball{ diameter: 5 };
    let mut sphere = Sphere{ diameter: 10 };
    // cannot do that:
    //ball = sphere;



    let a = 5i32;
    let b = 5i64;

    println!("{:?}", add(a, b));
}

fn add<T: Add, M: Add + TryInto<T>>(a: T, b: M) -> anyhow::Result<<T as Add>::Output>
where <M as TryInto<T>>::Error: std::fmt::Debug
{
    Ok(a + b.try_into().unwrap())
}