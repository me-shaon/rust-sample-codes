/* Simple Prime number generator Code */

fn main() {

    let mut prime_array = Vec::new();

    //Generating Prime numbers below than 100
    for i in 2..100 {
        let mut is_prime = true;

        for j in 2..i {
            if i%j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            prime_array.push(i);
        }
    }

    println!("Total Primes: {}", prime_array.len());

    //Printing all the prime numbers
    for i in &prime_array {
        println!("{}", i);
    }
}
