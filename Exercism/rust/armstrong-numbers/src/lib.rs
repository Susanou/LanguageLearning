pub fn is_armstrong_number(num: u32) -> bool {
    let n_str = num.to_string();
    let result: u32 = n_str.chars().map(|c| c.to_digit(10).unwrap()).map(|x| x.pow(n_str.len() as u32)).sum();
    result == num
}
