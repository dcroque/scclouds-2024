use scclouds_2024::{fibonacci, prime_check, prime_find};

use structopt::StructOpt;

#[derive(StructOpt)]
struct FibonacciCalc {
    #[structopt(short = "n", long = "nth", about = "n should be >=0")]
    nth: u64,
    #[structopt(short = "f", long = "function", default_value = "linear")]
    function: String,
}

#[derive(StructOpt)]
struct PrimeFinder {
    #[structopt(short = "n", long = "number", about = "n < 2 wields no results")]
    number: u64,
    #[structopt(short = "f", long = "function", default_value = "functional")]
    function: String,
}

#[derive(StructOpt)]
struct PrimeChecker {
    #[structopt(short = "n", long = "number")]
    number: u64,
    #[structopt(short = "f", long = "function", default_value = "functional")]
    function: String,
}

#[derive(StructOpt)]
enum Cli {
    #[structopt(
        about = "Find the nth Fibonacci number. Function options: linear, recursive, binet"
    )]
    Fibonacci(FibonacciCalc),
    #[structopt(
        about = "Find all the primes smaller than number. Function options: linear, recursive, functional"
    )]
    PrimeFinder(PrimeFinder),
    #[structopt(
        about = "Check if number is prime. Function options: linear, recursive, functional"
    )]
    PrimeChecker(PrimeChecker),
}

fn main() {
    let cli = Cli::from_args();

    match cli {
        Cli::Fibonacci(args) => {
            let result = match args.function.as_str() {
                "linear" => fibonacci::linear(args.nth),
                "recursive" => fibonacci::recursive(args.nth),
                "binet" => fibonacci::binet(args.nth),
                f => {
                    println!("\tFailed to process Fibonacci with function {}", f);
                    println!("\tOptions for function: linear, recursive, binet");
                    return;
                }
            };

            match result {
                None => println!(
                    "\tFailed to get {}th element: Values above 93 causes overflow",
                    args.nth
                ),
                Some(x) => println!("\tThe {}th Fibonacci element is {}", args.nth, x),
            }
        }
        Cli::PrimeFinder(args) => {
            let result = match args.function.as_str() {
                "linear" => prime_find::linear(args.number),
                "recursive" => prime_find::recursive(args.number),
                "functional" => prime_find::functional(args.number),
                f => {
                    println!("\tFailed to process PrimeFinder with function {}", f);
                    println!("\tOptions for function: linear, recursive, functional");
                    return;
                }
            };

            match result.len() {
                0 => println!("\tSorry but there is no primes until 2"),
                1 => println!("\tFound 1 prime number: {:.?}", result),
                x => println!("\tFound {} prime numbers: {:.?}", x, result),
            }
        }
        Cli::PrimeChecker(args) => {
            let result = match args.function.as_str() {
                "linear" => prime_check::linear(args.number),
                "recursive" => prime_check::recursive(args.number),
                "functional" => prime_check::functional(args.number),
                f => {
                    println!("\tFailed to process PrimeChecker with function {}", f);
                    println!("\tOptions for function: linear, recursive, functional");
                    return;
                }
            };

            match result {
                true => println!("\tThe number {} is prime!", args.number),
                false => println!("\tThe number {} is not prime!", args.number),
            }
        }
    }
}
