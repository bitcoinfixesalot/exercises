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
            append_encoded(&mut encoded, &old_series, &count)
        }
        count = 1;
        series = Some(c);
    }

    if let Some(old_series) = series {
        append_encoded(&mut encoded, &old_series, &count)
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

        decoded.push_str(&(c.to_string().repeat(count.parse().unwrap_or(1))));
        count.clear();
    }

    decoded
}

fn append_encoded(encoded: &mut String, letter: &char, count: &i32) {
    if *count > 1 {
        encoded.push_str(&count.to_string());
    }
    if *count > 0 {
        encoded.push(*letter);
    }
}
