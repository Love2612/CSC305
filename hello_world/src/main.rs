///associate greetings module with this crate
mod greetings;
mod how_you_hold_data_for_operations;


extern crate hello_world_lib;

 /// Optionally load each member of greetings
 /*use greetings::default_greeting;
 use greetings::spanish;
 use greetings::french; */

///Alternatively use * to load them all
// use greetings::*;

/// Alternatively load them in one line
use greetings::{english, spanish, french};
use crate::how_you_hold_data_for_operations::{derived::user_defined::{hera, enumm}, primitives::{scalar::{perform_operation, perform_operation1, perform_operation2, perform_operation3, perform_operation4, perform_operation5}, compound::{comp, comp2, comp3, mayne, maine}}};

struct Rect {
    length: f32,
    width: f32,
    name: &'static str,
}

impl Rect {
    fn new(length: f32, width: f32, name: &'static str) -> Self {
        Rect { length, width, name }
    }

    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn perimeter(&self) -> f32 {
        2.0 * (self.length + self.width)
    }
}

struct Circle {
    radius: f32,
}

impl Circle {
    fn new(radius: f32) -> Self {
        Circle { radius }
    }

    fn area(&self) -> f32 {
        3.1415926 * self.radius * self.radius
    }

    fn perimeter(&self) -> f32 {
        2.0 * 3.1415926 * self.radius
    }
}

struct Triangle {
    side_a: f32,
    side_b: f32,
    side_c: f32,
}

impl Triangle {
    fn new(side_a: f32, side_b: f32, side_c: f32) -> Self {
        Triangle { side_a, side_b, side_c }
    }

    fn area(&self) -> f32 {
        // Implement the area calculation for a triangle
        // (You can use Heron's formula or any other suitable formula)
        let s = (self.side_a + self.side_b + self.side_c) / 2.0;
        f32::sqrt(s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c))
    }

    fn perimeter(&self) -> f32 {
        self.side_a + self.side_b + self.side_c
    }
}

fn main() {
    // let mut greeting: &str = "Hello there";  it would not work if you do not add the mut because default declared variables are immutable

    println!("Hello, world!");
    // println!("{}", default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", english::default_greeting());
    println!("{}", english::default_greeting2());
    println!("{}", hello_world_lib::greeting_from_lib());
    println!("{}", perform_operation());
    println!("{}", perform_operation1());
    println!("{}", perform_operation2());
    println!("{}", perform_operation3());
    println!("{}", perform_operation4());
    perform_operation5();
    comp();
    comp2();
    comp3();
    mayne();
    maine();
    hera();
    enumm();

    // Create instances of the new structs and use their methods
let rectangle = Rect::new(3.0, 4.0, "Rectangle");
println!("Rectangle area: {}", rectangle.area());
println!("Rectangle perimeter: {}", rectangle.perimeter());

let circle = Circle { radius: 5.0 };
println!("Circle area: {}", circle.area());
println!("Circle perimeter: {}", circle.perimeter());

let triangle = Triangle {
    side_a: 3.0,
    side_b: 4.0,
    side_c: 5.0,
};
println!("Triangle area: {}", triangle.area());
println!("Triangle perimeter: {}", triangle.perimeter());

}

