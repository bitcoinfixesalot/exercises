pub fn rotate(input: &str, key: i8) -> String {
    let key = ((key % 26) + if key < 0 { 26 } else { 0 }) as u8;

    String::from_utf8(
        input
            .bytes()
            .map(
                |c| match (c.is_ascii_alphabetic(), c.is_ascii_lowercase()) {
                    (true, true) => (c + key - b'a') % 26 + (b'a'),
                    (true, false) => (c + key - b'A') % 26 + (b'A'),
                    _ => c,
                },
            )
            .collect(),
    )
    .unwrap()
}
