/// This code is practice from the "Programming Rust 2nd Edition" 
/// Chapter 2 - Concurrency

use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;
use std::str::FromStr;
use std::fs::File;
use std::env;

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

fn pixel_to_point(bounds: (usize, usize),
                pixel: (usize, usize),
                upper_left: Complex<f64>,
                lower_right: Complex<f64>) -> Complex<f64> 
{
    let (width, height) = (lower_right.re - upper_left.re,
                                upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64 // Pixel.1 increases as we go down but imaginary component increases as we go up
    }
}

fn render(pixels: &mut [u8],
        bounds: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1); // Check if pixels match image size bounds

    for row in 0..bounds.1{
        for column in 0..bounds.0{
            let point = pixel_to_point(bounds,(column,row), upper_left, lower_right); // Convert pixels to points
            pixels[row * bounds.0 + column] = match escape_time(point,255){ // If escape_time states that point belongs to the set
                None => 0, // Color as black
                Some(count) => 255 - count as u8 // Else assign darker colors to the numbers that took longer to escape the circle
            };
        }
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error>{
    let output = File::create(filename)?; // Creates and opens a file path

    let encoder = PNGEncoder::new(output); 
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?; // Each byte is an 8 bit grayscale value
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Get arguments
    if args.len() != 5 { // Make sure the arguments are inputted correctly
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT",
        args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35
        -1,0.20",
        args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
    .expect("error parsing image dimensions");

    let upper_left = parse_complex(&args[3])
    .expect("error parsing upper left corner point");

    let lower_right = parse_complex(&args[4])
    .expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1]; // vec![v;n] is a macro that creates a vector n elements long whos elements are initialized to v

    // render(&mut pixels, bounds, upper_left, lower_right); 

    // Concurrency in Rust using crossbeam crate
    let threads = 8;    // Declare number of threads to use
    let rows_per_band = bounds.1 / threads + 1; // Declare how many pixels in each band

    {
        // Use the chunks_mut to produce nonoverlapping slices of the buffer
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect(); // collect method builds a vector holding the mutable slices

        crossbeam::scope(|spawner| { // Create threads
            for (i, band) in bands.into_iter().enumerate() {

                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap();
    }

    write_image(&args[1], &pixels, bounds)
    .expect("error writing PNG file");
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

#[test]
fn test_pixel_to_point(){
    assert_eq!(pixel_to_point((100, 200), (25, 175), Complex{re: -1.0, im: 1.0}, Complex{re: 1.0, im: -1.0}), Complex{re: -0.5, im: -0.75});
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

// pixel.0 as f64: "pixel.0" refers to the first tuple element which is converted to f64 using "as f64"

// Using "?" in main wont work since main does not return a value

// | spawner |{}: a Rust closure that expects a single argument