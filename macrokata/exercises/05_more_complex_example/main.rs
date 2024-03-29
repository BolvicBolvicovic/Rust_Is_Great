////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: Create `for_2d!` macro here.

macro_rules! for_2d {
	($x:ident <$type:ty> in $X:expr, $y:ident <$type2:ty> in $Y:expr, {$code:expr}) => {
		for $x in $X {
			let $x:$type = $x;
			for $y in $Y {
				let $y: $type2 = $y;
				{ $code }
			}
		}
	};
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
        (Coordinate {x: col, y: row}).show()
    });

    let values = [1, 3, 5];

    for_2d!(x <u16> in values, y <u16> in values, {
        (Coordinate {x: x.into(), y: y.into()}).show()
    });
}
