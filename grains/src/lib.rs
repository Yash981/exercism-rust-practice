pub fn square(s: u32) -> u64 {
    let val = 2 as u64;
    return val.pow(s-1) as u64;
}

pub fn total() -> u64 {
    return (1..=64).map(square).sum();
}
