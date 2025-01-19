trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn print_area(shape: &dyn Shape) {
    println!("The area is {}", shape.area());
}

enum EnumShape {
    Circle,
    Rectangle,
}

fn create_shape(shape: EnumShape) -> Box<dyn Shape> {
    match shape {
        EnumShape::Circle => {
            Box::new(Circle {radius: 10.0})
        }
        EnumShape::Rectangle => {
            Box::new(Rectangle {width: 10.0, height: 5.0})
        }
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 4.0, height: 6.0 };

    print_area(&circle);
    print_area(&rectangle);

    let shape = create_shape(EnumShape::Circle);
    println!("Area: {}", shape.area());

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 4.0, height: 6.0 }),
    ];

    for shape in shapes {
        println!("Area: {}", shape.area());
    }
}
