pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in string.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' | '}' | ']' => if stack.pop() != Some(opener(c)) { return false } else { () },
            _ => ()
        }
    }

    stack.is_empty()
}

fn opener(bracket: char) -> char {
    match bracket {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => panic!("unknown bracket {}", bracket),
    }
}
