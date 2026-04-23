use std::result;




                     

// the same code with mutable variables     
fn main() {
    let proceed = true;
    if proceed {
        println!("Hello, WSL!");
    } else {
        println!("Goodbye, WSL!");
    }

    let height = 1.95;
    if height > 1.85 {
        "You are tall!"
    } else if height == 1.85 {
       "You are of average height!"
    } else {                                
        "You are short!"
    };

    println!("Result: {}", height);
}