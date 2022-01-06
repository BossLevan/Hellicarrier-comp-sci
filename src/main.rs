//finding the modular multiplicative inverse of a given number and modulus pair

//adding this std::io moudule lets the program use inbuilt Rust input/output functions
//in this case we are using it to accept our number and modulus from the user as inputs
use std::io;


//Entry point into the code
fn main() 
    {   
        //initializing empty String variables for 
        //number and modulus going to be inputted
        let mut input_number = String::new();
        let mut input_modulus = String::new();

        //inputted variables converted to integers
        let number: i32;
        let modulus: i32;

        println!("Enter your number");
        
        //the match statement below accepts a user input and writes it to: input_number
        //match statements are similar to Switch/case
        //statements in other programming languages.
        match io::stdin().read_line(&mut input_number)
        {
            //if successful, match returns Ok below
            Ok(_p) => {
                println!("your number is {}", input_number);
            }
            //if not successful, match returns Err below
            Err(_error) => {
                println!("Please try again");
            }
        }

        println!("Enter your modulus");
        match io::stdin().read_line(&mut input_modulus)
        {
            Ok(_) => {
                println!("Great! your number is {}", input_modulus);
            }
            Err(_error) => {
                println!("Please try again");
            }

        }

        //assigning number & modulus variables to the corresponding inputs
        //errors if theres a problem
        number = input_number.trim().parse().expect("Input is not an integer");
        modulus = input_modulus.trim().parse().expect("Input is not an integer");

        //running the modular inverse closure.
        //Its a closure not a function so that it can capture 
        //variables within the context of the main function
        let modular_inverse = |number, modulus| -> i32 {
            let mut a = number;
            let mut m = modulus;
            let initial_modulus = m; 
            
            let mut y: i32 = 0;
            let mut x: i32 = 1;

            if m == 1
            //this function returns 0 because the modular multiplicative inverse of any number with modulus 1 is 0
            //1 is a perfect divisor for all integers
            {
                return 0;
            }

            //the while statement below runs the code in bracket on a loop as long
            //as 'a' remains greater that 1, the while statement runs
            while a > 1
            {
                
                let q = a / m ;

                let mut t = m;

                //using the Euclidean algorithm, we evaluate the remainder and divisors of a and m
                //we iterate with the newly assigned values until the value of a drops below 1.
                //The idea for Euclidean algorithms is that it that takes two integers ‘a’ and ‘m’, 
                //finds their gcd and also find ‘x’ and ‘y’ such that ax + my = gcd(a, m)
                m = a % m;
                a = t;

                t = y;

                //updating y and x and iterating
                y = x - q * y;

                x = t;
        
            }

            //if x is negative, to make it positive again:
            if x < 0
            {

                x = x + initial_modulus;
                println!("{} is the modular multiplicative inverse of {} with modulus {}", x, input_number, input_modulus);

                return x;
            }

            println!("{} is the modular multiplicative inverse {} with modulus {}", x, input_number, input_modulus);

            return 0;
        };

        //call the closure
        modular_inverse(number, modulus);
        
    }
