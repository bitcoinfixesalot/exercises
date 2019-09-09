pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '{' | '[' | '(' => stack.push(c),
            '}' | ']' | ')' => {
                if let Some(prev) = stack.pop() {
                    if prev == get_opening_bracket(c) {
                        continue;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}

fn get_opening_bracket(c: char) -> char {
    match c {
        '}' => '{',
        ']' => '[',
        ')' => '(',
        _ => panic!("{}", "incorect bracket"),
    }
}
