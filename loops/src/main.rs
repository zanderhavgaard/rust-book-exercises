fn main() {
    // simplest loop
    // loop {
    //     println!("looping");
    // }

    // loops can return values
    let mut counter = 0;
    let loop_counter_result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The result of the counter is: {loop_counter_result}");

    // handling nested loops with labels
    let mut count = 0;
    'counter: loop {
        println!("Coount: {count}");
        let mut remaining = 10;
        loop {
            println!("Remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counter;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("Number: {number}");
        number -= 1;
    }
    println!("End of countdown");

    // for loop
    let my_array = [1, 2, 3, 4, 5];
    println!("My_array: {my_array:?}");
    for element in my_array {
        println!("Element: {element}");
    }

    // for loop over range
    for number in 1..10 {
        println!("Number: {number}");
    }

    let number_range = 1..10;
    println!("Range? {number_range:?}");
    let number_range_reversed = number_range.rev();
    println!("Range reversed: {number_range_reversed:?}");
}
