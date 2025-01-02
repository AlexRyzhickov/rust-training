fn main() {
    // Endless loop
    // use std::{thread, time::Duration};
    // loop {
    //     // Puts the current thread to sleep for at least the specified amount of time.
    //     // https://doc.rust-lang.org/std/thread/fn.sleep.html
    //     thread::sleep(Duration::from_millis(4000));
    //     println!("again!");
    // }

    // Returning Values from Loops
    {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {result}");
    }
    // Loop Labels to Disambiguate Between Multiple Loops
    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
    }
    // Conditional Loops with while
    {
        let mut number = 3;
        while number != 0 {
            println!("{number}!");
            number -= 1;
        }
        println!("LIFTOFF!!!");
    }
    // Looping Through a Collection with while
    {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
        while index < a.len() {
            println!("the value is: {}", a[index]);
            index += 1;
        }
    }
    // Looping Through a Collection with for
    {
        let a = [10, 20, 30, 40, 50];
        for element in a {
            println!("the value is: {element}");
        }
    }
    {
        for number in (0..4) {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
    {
        for number in (2..6).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
}
