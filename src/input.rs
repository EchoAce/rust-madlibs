use std::io;


fn user_input(wordType: &str) -> &str {
    let input = io::stdin().read_line()
                           .ok()
                           .expect("Failed to read a line.");

    let input_num: Option<int> = from_str(input.as_slice().trim());

    let isNumber = false;
    match input_num {
        Some(number) => number,
        None         => isNumber = false
    }

    if wordType == "number" && !isNumber {
        println!("This is not a number");
        "no value"
    }

    let index = 0;

    match input.find_str(" or ") {
        Some(number) => index = number,
        None => index = 0
    }

    if index > 0 {

    }
}
