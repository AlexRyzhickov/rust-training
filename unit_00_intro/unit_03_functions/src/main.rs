fn main() {
    println!("Hello, world!");

    another_function();
    print_number(5);
    print_labeled_measurement(5, 'h');
    let x = get_number(5);
    print!("x is {}", x);
    let y = {
        let x = 3;
        x + 1
    };
    print!("y is {}", y);
}

fn another_function() {
    println!("Another function.");
}

fn print_number(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn get_number(value: i32) -> i32 {
    value + 2
}
