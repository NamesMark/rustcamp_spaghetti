use smart_default::SmartDefault;

#[derive(Default, Debug)]
enum BookType {
    Soft,
    #[default]
    Hard,
    Electronic(usize),
}

#[derive(Debug)]
#[derive(SmartDefault)]
struct Algorithm {
    #[default = 12]
    exhaustiveness: i32,
    #[default = "just normal"]
    beauty: String,
    shortness: String,
}

impl Algorithm {
    fn new() -> Self {
        Self::default()
    }
}


fn main () {
    let algo_old = Algorithm::default();
    
    let algo = Algorithm { beauty: "majestic beast".to_string(), ..Default::default() };
    
    println!("{}", algo_old.beauty);
    println!("{:?}", algo);
    movefn(algo_old);
    
    
    println!("{:?}", BookType::default());
}

fn movefn(_algo: Algorithm) {
    ()
}

