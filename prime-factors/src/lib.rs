
pub fn factors(mut n: u64) -> Vec<u64> {
    let max_val = 100_00_00;
    let mut is_prime: Vec<bool> = vec![true;max_val+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=max_val{
        if is_prime[i]{
            let mut j = i * i;
            while j <= max_val {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let primes:Vec<u64> = (0..is_prime.len()).filter(|&x| is_prime[x]).map(|x| x as u64).collect();

    let mut factors: Vec<u64> = vec![];
    let mut i = 0;
    while n > 1 && i < primes.len() {
        if n % primes[i] == 0 {
            factors.push(primes[i]);
            n /= primes[i];
        } else {
            i += 1;
        }
    }
    factors

}
