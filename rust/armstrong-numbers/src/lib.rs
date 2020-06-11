const RADIX: u32 = 10;

pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let length = num_str.len() as u32;

    num_str
        .chars()
        .map(|n| n.to_digit(RADIX).unwrap().pow(length))
        .sum::<u32>()
        == num
}
