fn main() {
    let test1 = "ABC123DEF".to_string();
    let test2 = "123ABC".to_string();

    // Assertions for the test cases
    assert_eq!(q1_parser(test1), true);
    assert_eq!(q1_parser(test2), false);

    let result = square(12);
    println!("result is {},", result);

    let celcius_temp = 100.0;
    let farenheight_temp = celcius_to_farenheit(celcius_temp);
    assert_eq!(farenheight_temp, 212.0);
    print!("{} degrees Celcius is {} degrees Farenheit", celcius_temp, farenheight_temp);   

}

// Utility method that takes a character and returns true if digit 
// or upper case letter.
fn is_uppercase_or_digit(c: char) -> bool {
    is_uppercase_letter(c) || (c >= '0' && c <= '9')
}

// Utility method that takes a character and returns true if it is 
// an upper case letter.
fn is_uppercase_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}
//functions must be used or else code will not run on my machine VSCode
fn q1_parser(text: String) -> bool {
   
    // make String into char vector
    let characters_array: Vec<char> = text.chars().collect();

    if characters_array.len() < 2 {
        return false;
    }

    // for each character in the character vector do...
    // where i starts at 1 and increments for each iteration
    for (i, character) in characters_array.iter().enumerate() {
        // YOUR CODE GOES HERE
        if i < 2 {
            //Check if the first two characters are uppercase letters
            if !is_uppercase_letter(*character) {
                return false;

            }
            //Check if the first two characters are uppercase or digits
        } else if !is_uppercase_or_digit(*character) {
            return false;
        }
    }
    true



}   

fn square(x: i32) -> i32 {
    println!("squaring {}", x);
    x * x
}

fn celcius_to_farenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
