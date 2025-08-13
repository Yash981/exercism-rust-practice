pub fn nth(n: u32) -> u32 {
    const N: usize = 100_00000; // 10^6
    let mut is_prime: Vec<bool> = vec![true; N + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=N {
        let mut j = i * i;
        while j <= N {
            is_prime[j] = false;
            j += i;
        }
    }
    let mut primes: Vec<u32> = vec![];
    for (i,x) in is_prime.iter().enumerate(){
        if *x {
                primes.push(i as u32);
        }
        if primes.len() > n as usize {
            break;
        }
    }
    primes[n as usize]
}
