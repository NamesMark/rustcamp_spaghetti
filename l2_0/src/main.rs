
trait Flying {
    fn fly(&self) {
        println!("{} flied", std::any::type_name::<Self>());
    }
}
trait Walking {
    fn walk(&self) {
        println!("{} walked", std::any::type_name::<Self>());
    }
}
trait Running {
    fn run(&self) {
        println!("{} runned", std::any::type_name::<Self>());
    }
}
trait Riding {
    fn ride(&self) {
        println!("{} ridden a bike", std::any::type_name::<Self>());
    }
}

struct Birb;
impl Flying for Birb {}
impl Walking for Birb {}
impl Running for Birb {}
struct Hooman;
impl Walking for Hooman {}
impl Running for Hooman {}

impl Riding for Hooman {}

impl WalkerRunner for Birb {}
impl WalkerRunner for Hooman {}

trait WalkerRunner: Walking + Running {}
type Grounded = dyn WalkerRunner;

fn main() {
    let vec: Vec<Box<dyn WalkerRunner>> = vec![Box::new(Birb{}), Box::new(Hooman{})];

    for entity in vec {
        do_something(entity);
    }
}

fn do_something(entity: Box<Grounded>) {
    entity.walk();
    entity.run();
}
