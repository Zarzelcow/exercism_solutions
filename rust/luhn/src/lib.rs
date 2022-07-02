/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let stripped =  code.chars().filter(|c| *c != ' ').collect::<Vec<_>>();
    if stripped.len() <= 1 { return false; }
    let mut sum = 0;
    let parity = stripped.len() % 2;
    for (index, char) in stripped.iter().enumerate() {
        if !('0'..='9').contains(char) { return false; }
        let mut digit = char.to_digit(10).unwrap();
        if index % 2 == parity {
            digit *= 2;
        }
        if digit > 9 {
            digit -= 9;
        }
        sum += digit;
    }
    sum % 10 == 0
}