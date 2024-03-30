fn main() {
    println!("Hello, world!");

    another_function();
    int_function(1);
    int_function(2);

    print_labeled_measurement(1, 'h');
    print_labeled_measurement(2, 'k');

    let five = five();
    println!("five from a function: {five}");

    let _two = plus_one(1);
}

fn another_function() {
    println!("Hello from another function!")
}

fn int_function(x: i32) {
    println!("The value of paramter x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn five() -> i32 {
    // 5 is an expression and is returned
    // note that there is no semicolon after the return expression
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
