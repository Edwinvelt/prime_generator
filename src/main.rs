// By listing the first six prime numbers: 2, 3, 5, 7, 11 and 13, we can see that the 6th prime is 13.
// What is the 10001st prime number?

fn main() {
    // Default 10.001
    let limit = 1000000000; // 1b in 66s
    let result = sieve_of_atkin(limit);
    println!("{:?}", result);
}

// !! This is mostly a interpretation of a explanation (I didn't really understand) of sieve of atkin.
fn sieve_of_atkin(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![false; limit + 1];
    let sqrt_limit = (limit as f64).sqrt() as usize;

    let squares: Vec<usize> = (0..=sqrt_limit).map(|x| x * x).collect();

    // ! The algorithm completely ignores any numbers with remainder modulo 60 that is divisible by two, three, or five,
    // ! since numbers with a modulo 60 remainder divisible by one of these three primes are themselves divisible by that prime.

    // Alg step 1 | Solution to: 4*x^2 + y^2 = n is odd and the number is squarefree
    (1..=sqrt_limit).for_each(|x| {
        (1..=sqrt_limit).for_each(|y| {
            let n = 4 * squares[x] + squares[y];
            if n <= limit && matches!(n % 60, 1 | 13 | 17 | 29 | 37 | 41 | 49 | 53) {
                is_prime[n] = !is_prime[n];
            }
        });
    });

    // Alg step 2 | 3*x^2 + y^2 = n is odd and n is squarefree
    (1..=sqrt_limit).for_each(|x| {
        (1..=sqrt_limit).for_each(|y| {
            let n = 3 * squares[x] + squares[y];
            if n <= limit && matches!(n % 60, 7 | 19 | 31 | 43) {
                is_prime[n] = !is_prime[n];
            }
        });
    });

    // Alg step 3 | Solution to: 3*x^2 - y^2 = n is odd and n is squarefree
    (1..=sqrt_limit).for_each(|x| {
        (1..x).for_each(|y| {
            let n = 3 * squares[x] - squares[y];
            if n <= limit && matches!(n % 60, 11 | 23 | 47 | 59) {
                is_prime[n] = !is_prime[n];
            }
        });
    });

    // Eliminate composites by marking their multiples as non-prime
    (5..=sqrt_limit).for_each(|n| {
        if is_prime[n] {
            let mut k = n * n;
            while k <= limit {
                is_prime[k] = false;
                k += n * n;
            }
        }
    });

    // Collect all primes
    let mut primes = vec![2, 3];
    primes.extend((5..=limit).filter(|&n| is_prime[n]));
    primes
}

