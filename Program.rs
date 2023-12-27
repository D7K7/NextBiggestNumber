fn next_bigger_number(n: i64) -> Option<i64> {
    let mut digits = n.to_string().chars().collect::<Vec<_>>();
    let len = digits.len();

    let i = match (1..len).rev().find(|&i| digits[i - 1] < digits[i]) {
        Some(i) => i,
        None => return None,
    };

    let j = (i..len).rev().find(|&j| digits[j] > digits[i - 1]).unwrap();

    digits.swap(i - 1, j);

    digits[i..].sort_unstable();
  
    let result_str = digits.into_iter().collect::<String>();
    match result_str.parse::<i64>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
