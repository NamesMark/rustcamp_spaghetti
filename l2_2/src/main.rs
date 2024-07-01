struct Hooman {}
impl Walking for Hooman {}

trait Walking {
    fn walk(&self, i: usize) {
        println!("{i}: {} walked", std::any::type_name::<Self>());
    }
}

trait Flying {
    fn fly(&self, i: usize) {
        println!("{i}: {} flied", std::any::type_name::<Self>());
    }
}

struct Birb {}
impl Walking for Birb {}
impl Flying for Birb {
    fn fly(&self, i: usize) {
        println!("{i}: BIRDIE flied");
    }
}
impl WalkingFlying for Birb {
    fn fly(&self, i: usize) {
        println!("{i}: WalkingFlying: BIRDIE flied");
    }
}

trait WalkingFlying: Walking + Flying {
    fn fly(&self, i: usize);
}

//impl !Send for Birb {}

fn main() {
    let animals: Vec<Box<dyn WalkingFlying>> = vec![Box::new(Birb{}), Box::new(Birb{})];

    for (i, animal) in animals.iter().enumerate() {
        animal.walk(i);
        //(animal as Birb).fly(i);
    }
}

trait Getter {
    fn get_article(&self) -> Result<String, ()>;
}

struct HTTPGetter {
    url: &'static str,
}

impl Getter for HTTPGetter {
    fn get_article(&self) -> Result<String, ()> {
        Ok("Article".to_string())
    }
}