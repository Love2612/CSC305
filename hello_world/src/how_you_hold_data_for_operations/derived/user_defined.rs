// Structures
// An attribute to hide warnings for unused code.
#![allow(dead_code)]

use core::cmp::Ordering; //Used dor comparison of value sizes 

pub enum Comp { //Enumerate Comparison
    LessThan,
    GreaterThan,
    Equal,
}

pub enum Gender { //Enumerate Gender
    Male,
    Female,
}


#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
} //Has fields but dictionary type fields

// A unit struct
struct Unit;
//traits like Error
//has no state of their own but useful for implementing some traits

// A tuple struct
struct Pair(f32, f32); // No named fields but has fields

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
} // a struct with two fields holding x and y coordinates. This struct Point is the very definition of user-defined data type

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

pub(crate) fn hera() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right, 
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1.0, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}  


// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

pub fn enumm() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

//Let's work on user-defined traits. Traits enable us achieve polymorphism.
//We are designing Shape below for the purpose of 
//specifying all expected functions and methods in any struct that implements Shape.
trait Shape {
    fn new(length: f32, width: f32, name: &'static str) -> Self;
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn set_length(&mut self, length: f32);
    fn get_length(&self) -> f32;
    fn set_width(&mut self, width: f32);
    fn get_width(&self) -> f32;
    fn set_name(&mut self, name: &'static str);
    fn get_name(&self) -> &str;
}
//The use of 'static lifetime above ensures that our
//compiler is clear about the availability of those values, as they are borrowed.
//static will be available throughout the lifetime of the program.

///Use Default to specify the availability of default instance creation without values passed for property
#[derive(Default, Debug, Clone)]
pub struct Rect {
    length: f32,
    width: f32,
    name: &'static str,
}

impl Rect {
    //default default() function. Will override derived default if any. 
    fn default() -> Self {
        Rect {
            length: 1.0,
            width: 1.0,
            name: "default_name",
        }
    }
}

impl Shape for Rect {
    //Associated function used to create a new Shape
    fn new(length: f32, width: f32, name: &'static str) -> Self {
        Rect {
            length,
            width,
            name,
        }
    }

    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn perimeter(&self) -> f32 {
        2.0 * (self.length + self.width)
    }

    fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

//implement Partial Eq
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}


//A conversion implementation into String
//Expects a string slice with length, width, name, separated by commas
impl From<&'static str> for Rect {
    fn from(s: &'static str) -> Rect {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let width = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Rect { length, width, name: &name }
    }
}

impl Into<String> for Rect {
    fn into(self) -> String {
        format!("My name is {} and my area is {}.", self.name, self.area())
    }
}

pub struct Circle {
    radius: f32
}

impl Into<Circle> for Rect {
    fn into(self) -> Circle {
        let radius: f32 = f32::sqrt(self.area() / 3.1415926);
        Circle { radius }
    }
}


impl Shape for Circle {
    fn new(_length: f32, _width: f32, _name: &'static str) -> Self {
        Circle { radius: 0.0 }
    }

    fn area(&self) -> f32 {
        // Assuming the area of a circle is an integer
        (3.1415926 * self.radius * self.radius) as f32
    }

    fn perimeter(&self) -> f32 {
        // Assuming the perimeter of a circle is an integer
        (2.0 * 3.1415926 * self.radius) as f32
    }

    fn set_length(&mut self, _length: f32) {
        // Circles do not have a length, so this is a no-op
    }

    fn get_length(&self) -> f32 {
        // Circles do not have a length, return a default value
        0.0
    }

    fn set_width(&mut self, _width: f32) {
        // Circles do not have a width, so this is a no-op
    }

    fn get_width(&self) -> f32 {
        // Circles do not have a width, return a default value
        0.0
    }

    fn set_name(&mut self, _name: &'static str) {
        // Circles do not have a name, so this is a no-op
    }

    fn get_name(&self) -> &str {
        // Circles do not have a name, return a default value
        ""
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Circle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.area().partial_cmp(&other.area())
    }
}

impl From<&'static str> for Circle {
    fn from(s: &'static str) -> Circle {
        let mut parts = s.split(',');
        let radius = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        Circle { radius }
    }
}


impl Into<String> for Circle {
    fn into(self) -> String {
        format!("My radius is {} and my area is {}.", self.radius, self.area())
    }
}

pub struct Triangle {
    side_a: f32,
    side_b: f32,
    side_c: f32,
}

impl Shape for Triangle {
    fn new(__length: f32, __width: f32, __name: &'static str) -> Self {
        Triangle { side_a: 0.0,
             side_b: 0.0,
              side_c: 0.0}
    }

    fn area(&self) -> f32 {
        // Assuming the triangle is valid (follows triangle inequality theorem)
        let s = (self.side_a + self.side_b + self.side_c) / 2.0;
        f32::sqrt(s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c))
    }

    fn perimeter(&self) -> f32 {
        self.side_a + self.side_b + self.side_c
    }

    fn set_length(&mut self, _length: f32) {
        // Triangles do not have a length
    }

    fn get_length(&self) -> f32 {
        // Triangles do not have a length
        0.0
    }

    fn set_width(&mut self, _width: f32) {
        // Triangles do not have a width
    }

    fn get_width(&self) -> f32 {
        // Triangles do not have a width
        0.0
    }

    fn set_name(&mut self, _name: &'static str) {
        // Triangles do not have a name
    }

    fn get_name(&self) -> &str {
        // Triangles do not have a name
        ""
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Triangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }
}

impl From<&'static str> for Triangle {
    fn from(s: &'static str) -> Triangle {
        let mut parts = s.split(',');
        let side_a = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let side_b = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let side_c = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        Triangle { side_a, side_b, side_c }
    }
}

impl Into<String> for Triangle {
    fn into(self) -> String {
        format!(
            "My sides are {}, {}, and {}. My area is {}.",
            self.side_a, self.side_b, self.side_c, self.area()
        )
    }
}
