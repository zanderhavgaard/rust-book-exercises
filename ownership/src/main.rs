fn main() {
    let _my_string_literal = "hello";
    let mut my_mutable_string = String::from("hello");

    // append a string literal to the mutable string
    my_mutable_string.push_str(" world");

    println!("{}", my_mutable_string);

    // two valaues are pushed to the stack
    let x = 5;
    let y = x;

    println!("{},{}", x, y);

    // create a string
    let string1 = String::from("hello");
    // create another pointer to the same string data on the heap
    // in rust this is called a 'move' as string1 is invalidated
    // consequently string1 is no longer in scope!
    let string2 = string1;
    // actually make a copy of the string
    let string1_clone = string2.clone();

    println!("{},{}", string2, string1_clone);

    // page 88 in the pdf
}
