use std::ops::Add;

struct Ball {
    diameter: i32,
}
struct Sphere {
    diameter: i32,
}

fn main() {
    let mut ball = Ball { diameter: 5 };
    let mut sphere = Sphere { diameter: 10 };
    // cannot do that:
    //ball = sphere;

    let a: i32 = 5i32;
    let b: i64 = 5i64;

    println!("{:?}", add(a, b));
    println!("{:?}", add_2(a, b));
}

fn add<T, M>(a: T, b: M) -> Result<<T as Add>::Output, <M as TryInto<T>>::Error>
where
    T: Add,
    M: TryInto<T>,
{
    Ok(a + b.try_into()?)
}

fn add_2<T, M>(a: T, b: M) -> <M as Add>::Output
where
    T: Into<M>,
    M: Add,
{
    a.into() + b
}
