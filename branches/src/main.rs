fn main() {
    let number = 3;

    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    let res = greater_than_five(number);
    println!("{}", res);

    if number != 0 {
        println!("number is not zero");
    }

    can_number_modulo(4);
    can_number_modulo(3);
    can_number_modulo(2);
    can_number_modulo(1);

    let mut condition = true;
    let mut number2 = if condition { 5 } else { 6 };
    println!("{}", number2);
    condition = false;
    number2 = if condition { 5 } else { 6 };
    println!("{}", number2);
}

fn greater_than_five(number: i32) -> bool {
    number > 5
}

fn can_number_modulo(number: i32) {
    if number % 4 == 0 {
        println!("number can be divided by 4");
    } else if number % 3 == 0 {
        println!("number can be divded by 3");
    } else if number % 2 == 0 {
        println!("number can be divided by 2");
    } else {
        println!("number cannot be divided by 4, 3 or 2");
    }
}
