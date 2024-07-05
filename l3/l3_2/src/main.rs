fn main () {
    let result = fun();
    println!("{:?}", result);
    let result = (||{})();
    println!("{:?}", result);
    get_number(true);
}

fn fun() -> () {
    1; //-> ()
    println!("5"); //-> ()

}

fn get_number(predicate: bool) -> usize {
    if true {
        5
    } else {
        std::process::exit(0);
        loop {
            //forever
        } //-> !.into() -> usize 
    }
}