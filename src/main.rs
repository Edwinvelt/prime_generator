// By listing the first six prime numbers: 2, 3, 5, 7, 11 and 13, we can see that the 6th prime is 13.
// What is the 10001st prime number?

fn main() {
    // default: 10.001
    let limit = 200000;
    let result = sieve_of_atkin(limit);
    let mut prime_list: Vec<usize> = Vec::new();

    for prime in result {
        prime_list.push(prime);
    }
    println!("{:?}", prime_list);
}

// !! This is mostly a interpretation of a explanation (I didn't really understand) of sieve of atkin.
// !! I don't understand what the hell is happening...
fn sieve_of_atkin(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![false; limit + 1];
    let sqrt_limit = (limit as f64).sqrt() as usize;

    // ! The algorithm completely ignores any numbers with remainder modulo 60 that is divisible by two, three, or five,
    // ! since numbers with a modulo 60 remainder divisible by one of these three primes are themselves divisible by that prime.

    // FIXME: skipping numbers

    // Alg step 1 | Solution to: 4*x.pow(2) + y.pow(2) = n is odd and the number is squarefree
    // ? Testing with iterators instead of explicit for loops | No performance difference
    (1..=sqrt_limit).for_each(|x| {
        (1..=sqrt_limit).step_by(2).for_each(|y| {
            let n = 4 * x.pow(2) + y.pow(2);
            if n <= limit && matches!(n % 60, 1 | 13 | 17 | 29 | 37 | 41 | 49 | 53) {
                is_prime[n] = !is_prime[n];
            }
        });
    });

    // Alg step 2 | 3*x.pow(2) + y.pow(2) = n is odd and n is squarefree
    for x in (2..=sqrt_limit).step_by(2) {
        for y in (2..=sqrt_limit).step_by(2) {
            let n = 3 * x.pow(2) + y.pow(2);
            if n <= limit && matches!(n % 60, 7 | 19 | 31 | 43) {
                // sets n in prime too false, if it was true
                is_prime[n] = !is_prime[n];
            }
        }
    }

    // Alg step 3 | Solution to: 3x.pow2 - y.pow(2) = n is odd and n is squarefree
    for x in 2..=sqrt_limit {
        for y in (1..x).step_by(2) {
            let n = 3 * x.pow(2) - y.pow(2);
            if n <= limit && matches!(n % 60, 11 | 23 | 47 | 59) {
                is_prime[n] = !is_prime[n];
            }
        }
    }

    // Eliminate composites by sieving
    for n in 7..=sqrt_limit {
        if is_prime[n] {
            let n2 = n * n;
            for k in (n2..=limit).step_by(n2) {
                is_prime[k] = false;
            }
        }
    }

    // Collect Primes
    let mut primes = vec![2, 3, 5];
    for n in (7..=limit).step_by(2) {
        if is_prime[n] {
            primes.push(n);
        }
    }
    // End result
    primes
}
