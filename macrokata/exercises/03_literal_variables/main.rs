////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `math!()` macro.

macro_rules! math {
	($nb1:literal plus $nb2:literal) => { $nb1 + $nb2 };
	(square $nb:literal) => { $nb * $nb };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(math!(3 plus 5));
    print_result(math!(square 2));
}
