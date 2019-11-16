pub fn encode(source: &str) -> String {
    let mut encoded = String::new();
    let mut count = 0;
    let mut series: Option<char> = None;

    for c in source.chars() {
        if let Some(old_series) = series {
            if old_series == c {
                count += 1;
                continue;
            }

            if count > 1 {
                encoded.push_str(&count.to_string());
            }
            encoded.push(old_series);
        }
        count = 1;
        series = Some(c);
    }

    if let Some(old_series) = series {
        if count > 1 {
            encoded.push_str(&count.to_string());
        }
        encoded.push(old_series);
    }

    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut count = String::new();

    for c in source.chars() {
        if c.is_numeric() {
            count.push(c);
            continue;
        }

        if count.len() == 0 {
            count = "1".to_string();
        }

        decoded.push_str(&(c.to_string().repeat(count.parse::<usize>().unwrap())));
        count = String::new();
    }

    decoded
}
