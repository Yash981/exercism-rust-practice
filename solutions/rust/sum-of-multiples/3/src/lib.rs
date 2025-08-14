use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")

    let mut multiple_factors = HashSet::new();
    for &x in factors{
        if x < limit && x != 0 {
            let mut curr = x;
            while curr < limit {
                multiple_factors.insert(curr);
                curr += x
            }
        }

    } 

    return multiple_factors.iter().sum();

}
