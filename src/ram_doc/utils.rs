pub fn bytes_to_megabytes(bytes: u64) -> u64 {
    return bytes / 1024 / 1024;
}

pub fn format_number(num: u64) -> String {
    let num_str = num.to_string();
    let mut formatted_str = String::new();
    let mut count = 0;

    for c in num_str.chars().rev() {
        if count > 0 && count % 3 == 0 {
            formatted_str.push(',');
        }
        formatted_str.push(c);
        count += 1;
    }

    return formatted_str.chars().rev().collect();
}
