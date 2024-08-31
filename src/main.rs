/**
 * Tai Meade
 * 01/30/24
 * ITEC 320
 * The purpose of this project is to create a program that determines if an input of
 * numbers are prime, composite, or neither (individually).  If it is a composite:
 * the factors, and prime factorization will be printed to the screen.
 *
 *
 * NOTE: I am attempting the extra credit opportunity!
 *
 *
 * Help received:
 *
 * https://www.youtube.com/watch?v=c7nnAktNlh4 <----- used to learn how to increase
 * the efficiency of determining if a number is prime.
 *
 * https://www.geeksforgeeks.org/find-all-factors-of-a-natural-number/ <----- used
 * to help determine a faster way to find the factors of a number...changed up a bit
 *
 * Retrieving user input and iterating based on that (read_int() and parts of main())
 * gotten from Dr. Lahn in class.
 * 
 * Basic documentation on for loops/iterating through arrays
 * 
 */

fn main() {
    let num_inputs: u64 = read_int();
    // loop to read remaining numbers
    for _ in 0..num_inputs {
        let n: u64 = read_int();

        println!("{}", n); // first print statement for output
        if !is_prime(n) {
            print!("{n} is composite\n");
            print_factors(n);
            print_prime_factorization(n);
        }
        // Case in which n == 1
        else if n == 1 {
            print!("{n} is neither prime nor composite")
        }
        // Must be prime if it is not 1 and is_prime() returns false
        else {
            print!("{n} is prime");
        }

        println!("\n"); // for formatting
    }
}

use std::io;
fn read_int() -> u64 {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); // Store next line of input in String

    let input = input.trim().parse().expect("Line contained non-integer");
    return input;
}

fn is_prime(num: u64) -> bool {
    let highest_num: f64 = square_root(num as f64);
    if num != 1 {
        // Iterate through numbers 2..(num - 1)...if a factor is found
        // then print that the number is composite and return false
        for i in 2..((highest_num + 1.0) as u64) {
            if num % i == 0 {
                return false;
            }
        }
        // If no factor is found then the number must be prime...return true
        return true;
    }
    // Case in which the input number is 1...return true...addressed
    // later in main function
    else {
        return true;
    }
}

fn print_factors(num: u64) {
    let mut factors: [u64; 10000] = [0; 10000]; // 10000 is max num of factors supported

    let mut j = 0; // used to track index

    print!("The positive factors of {num} are: ");

    let mut i: u64 = 1;

    // Only check numbers up to the square root...these are the only necessary numbers
    // thanks to the new method of storing factors
    while i <= square_root(num as f64) as u64 {
        // Check if i is a factor of num
        if num % i == 0 {

            // if the quotient iss the same as the factor...only store it once
            if num / i == i {
                factors[j as usize] = i;
                j += 1;
            } 
            // otherwise, store i and the corresponding factor
            else {
                factors[j as usize] = i;
                factors[(j + 1) as usize] = num / i as u64; // place other factor in next spot
                j += 2;
            }
        }
        i += 1;
    }

    factors.sort(); // sort the factors so they print in the proper order

    for (i, factor) in factors.into_iter().enumerate() {
        // Handles printing the final number itself without a comma after it...and add a \n
        if i == 9999 {
            println!("{factor}");
            break;
        } else if factor != 0 {
            print!("{factor}, ");
        }
    }
}

// Find the prime factorization of a number
fn print_prime_factorization(num: u64) {
    let mut mutable_num = num;
    let mut temp_count: i32;
    let mut i = 2;
    print!("The prime factorization of {num} is: ");

    // Loop until the shadowed number is 1
    while mutable_num != 1 {

        temp_count = 0; // tracks the exponent

        if is_prime(i) {
            
            // Coninue dividing by lowest prime until no longer possible 
            while mutable_num % i == 0 {
                temp_count += 1;
                mutable_num /= i;
            }
        }
        // case when there is an exponent and prime factorization is complete
        if temp_count > 1 && mutable_num <= 1 {
            print!("{i} ** {temp_count}");
        } 
        // case when there is no exponent and prime factorization is complete
        else if temp_count == 1 && mutable_num <= 1 {
            print!("{i}");
        } 
        // case when there is an exponent and prime factorization is not complete
        else if temp_count > 1 {
            print!("{i} ** {temp_count} * ");
        } 
        // case when there is an exponent and prime factorization is not complete
        else if temp_count == 1 {
            print!("{i} * ");
        }

        i += 1;

    }
}

// Credit for the method to find the square root to https://www.geeksforgeeks.org/square-root-of-a-perfect-square/ 
// ...used the Babylonian method...an iterative process to estimating square roots
// NOTE: reused this code from a previous exercise 
fn square_root(num: f64) -> f64 {
    let mut x = num;
    let mut y = 1.0;
    let error_margin = 0.000001;

    while x - y as f64 > error_margin {
        x = (x + y as f64) / 2.0;
        y = num / x;
    }

    return x;
}

// My futile attempt to make an insertion sort algorithm in Rust lol
// fn insertion_sort(array: &mut [u64; 1000]) {
//     let mut key: u64 = 0;
//     let mut j: u64 = 0;

//     for i in 1..array.len() {
//         key = array[i as usize];

//         j = i - 1;
//         while j >= 0 && key < array[j as usize] {
//             array[j + 1 as usize] = array[j as usize];
//             j -= 1;
//         }
//         array[j + 1 as usize] = key;
//     }
// }
