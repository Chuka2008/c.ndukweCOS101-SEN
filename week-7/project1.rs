use std::io;

fn trapezium(){
	let mut h = String::new();
	let mut b1 = String::new();
	let mut b2 = String::new();

	println!("Enter height of trapezium");
	io::stdin().read_line(&mut h).expect("Not a valid String");
	let h:f32 = h.trim().parse().expect("Not a valid Number");

	println!("Enter first base of trapezium");
	io::stdin().read_line(&mut b1).expect("Not a valid String");
	let b1:f32 = b1.trim().parse().expect("Not a valid Number");

	println!("Enter second base of trapezium");
	io::stdin().read_line(&mut b2).expect("Not a valid String");
	let b2:f32 = b2.trim().parse().expect("Not a valid Number");

	let area:f32 = h/2.0 * (b1 + b2);

	println!("area of trapezium: {}",area);
}

fn rhombus(){
	let mut d1 = String::new();
	let mut d2 = String::new();

	println!("Enter first diagonal");
	io::stdin().read_line(&mut d1).expect("Not a valid String");
	let d1:f32 = d1.trim().parse().expect("Not a valid Number");

	println!("Enter second diagonal");
	io::stdin().read_line(&mut d2).expect("Not a valid String");
	let d2:f32 = d2.trim().parse().expect("Not a valid Number");

	let area:f32 = 0.5 * d1 * d2;

	println!("area of rhombus: {}",area );
}

fn parallelogram(){
	let mut base = String::new();
	let mut alt = String::new();

	println!("Enter base of parallelogram");
	io::stdin().read_line(&mut base).expect("Not a valid String");
	let base:f32 = base.trim().parse().expect("Not a valid Number");

	println!("Enter altitude of parallelogram");
	io::stdin().read_line(&mut alt).expect("Not a valid String");
	let alt:f32 = alt.trim().parse().expect("Not a valid Number");

	let area:f32 = base * alt;

	println!("area of parallelogram: {}",area);
}

fn cube(){
	let mut side = String::new();

	println!("Enter length of side of cube");
	io::stdin().read_line(&mut side).expect("Not a valid String");
	let side:f32 = side.trim().parse().expect("Not a valid Number");

	let area:f32 = 6.0 * side * side;

	println!("area of cube: {}",area ); 
}

fn cylinder(){
	let mut rad = String::new();
	let mut height = String::new();
	let pi = 3.142

	println!("Enter radius of cylinder");
	io::stdin().read_line(&mut rad).expect("Not a valid String");
	let side:f32 = rad.trim().parse().expect("Not a valid Number");

	println!("Enter height of cylinder");
	io::stdin().read_line(&mut height).expect("Not a valid String");
	let side:f32 = height.trim().parse().expect("Not a valid Number");

	let area:f32 = pi * rad * height

	println!("area of cylinder: {}",area);
}
fn main() {
	println!("Welcome to my calculator! \nWhich problem would you like to solve today?");

	let mut input = String::new();

	println!("Enter one of the following; \n1.Area of trapezium \n2.Area of rhombus \n3.Area of parallelogram \n4.Area of cube \n5.Area of Cylinder");
	io::stdin().read_line(&mut input).expect("Not a valid String");
	let input:i32 = input.trim().parse().expect("Invalid Number");

	if input == 1{
		trapezium();
	}
	else if input == 2{
		rhombus();
	}
	else if input == 3{
		parallelogram();
	}
	else if input == 4{
		cube();
	}
	else if input = 5{
		cylinder();
	}
	else{
		println!("Invalid input");
	};

}