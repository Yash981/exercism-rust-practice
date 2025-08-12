pub fn is_armstrong_number(num: u32) -> bool {
    let str_num = num.to_string();
    let n = str_num.len() as u32;
    let mut ans = 0;
    for digit in str_num.chars().map(|x| x.to_digit(10).unwrap()) {
        ans += digit.pow(n)
    }
    return ans == num;
}
