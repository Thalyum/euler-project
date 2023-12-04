use primes::is_prime;

fn main() {
    let range = 10000 - (2 * 3330);
    for i in 1000..range {
        let number_1 = i;
        let number_2 = i + 3330;
        let number_3 = i + (2 * 3330);

        // check each numer is a permutation of the other
        let mut n1: Vec<char> = number_1.to_string().chars().collect();
        n1.sort();
        let mut n2: Vec<char> = number_2.to_string().chars().collect();
        n2.sort();

        if n1 != n2 {
            continue;
        }

        let mut n3: Vec<char> = number_3.to_string().chars().collect();
        n3.sort();

        if n1 != n3 {
            continue;
        }

        // check they are prime
        if !is_prime(number_1) || !is_prime(number_2) || !is_prime(number_3) {
            continue;
        }

        println!("{} - {} - {}", number_1, number_2, number_3);
    }
}
