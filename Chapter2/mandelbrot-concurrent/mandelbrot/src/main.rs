/// This code is practice from the "Programming Rust 2nd Edition" 
/// Chapter 2 - Concurrency

use num::Complex;
use std::str::FromStr;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> { 
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in 0..limit {
        if z.norm_sqr() > 4.0 { // Compare distance of z from 4 (2^2) to see if it left the circle
            return Some(i);     // Return the iteration "i" at which z leaves the circle
        }
        z = z * z + c;
    }

    None        // Return nothing if z is in the set
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)>{
    match s.find(separator){ // Searches the string for a character that matches the separator
        None => None, // If no separator, then return none
        Some(index) => { // If separator exists, return its index within string
            match(T::from_str(&s[..index]), T::from_str(&s[index+1..])){ // take slices preceeding and following the separator
                (Ok(l), Ok(r)) => Some((l, r)), // If both match the value type T, then return values
                _ => None   // "_" this is a wildcard pattern that can match any other value, so return nothing if so
            }
        }
    }
}

fn parse_complex(s: &str) -> Option<Complex<f64>>{
    match parse_pair(s, ','){
        Some((re, im)) => Some(Complex{re, im}),
        None => None
    }
}

fn main(){

}

// --------------------- Unit Tests --------------------- //
#[test]
fn test_parse_pair(){
    assert_eq!(parse_pair::<i32>("",    ','), None);
    assert_eq!(parse_pair::<i32>("10,",    ','), None);
    assert_eq!(parse_pair::<i32>(",10",    ','), None);
    assert_eq!(parse_pair::<i32>("10,20",    ','), Some((10,20)));
    assert_eq!(parse_pair::<i32>("10,20xy",    ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5",    'x'), Some((0.5, 1.5)));
}

#[test]
fn test_parse_complex(){
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex{re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex(",-0.0625"), None);
    assert_eq!(parse_complex("comma"), None);
}

// --------------------- Notes --------------------- //
// fn escape_time()
    // Options:
        /*enum Option<T>{
            None,
            Some(T),
        } */
        // An enumerated type that is used to represent either a value or the abscense of a value
        // T can be any type

    // for i in 0..limit: 
        // loop over the range of integers starting with 0 up to but not including "limit"

    // z.norm_sqr():
        // Returns the square of z's distance from origin

    // Documentation comments always start with "///" and end with the same characters 
        // It also goes above all code
        // Used by Rust compiler to produce online documentation
// fn parse_pair()
    // <T: FromStr>: for any type T that implements the FromStr trait...

// match is similar to the "switch" in C/C++

// #[test] needs to be above each test function you create