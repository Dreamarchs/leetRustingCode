use std::collections::VecDeque;
fn main() {
    let a = b'(';
    let b = b'[';
    let c = b'{';
    let d = b')';
    let e = b']';
    let f = b'}';
    println!("{} {} {} {} {} {}", a, b, c, d, e, f);

    let string = "([}}])";

    // // First pass at leetcode problem Valid Parentheses
    // let bytes = string.as_bytes();
    // println!("Bytes: {:?}", bytes);
    // let mut stack: Vec<u8> = Vec::new();
    //
    // for(i, &item) in bytes.iter().enumerate() {
    //
    //     if matches!(item as u8, b'{' | b'[' | b'(') {
    //         println!("Found '{item}'");
    //         stack.push(item);
    //     }
    //
    //     if matches!(item, b'}' | b']' | b')') {
    //         if stack.len() >= 1 {
    //             let lastChar = stack[stack.len() - 1];
    //             println!("Found '{item}'");
    //             println!("Comparing '{lastChar}' with '{item}'");
    //             if (item).abs_diff(lastChar) <= 2 {
    //                 stack.pop();
    //             } else {
    //                 return println!("{} is not balanced", string);
    //             }
    //         } else {return println!("{} is not balanced", string)}
    //     }
    // }
    // if stack.len() == 0 {return println!("Stack is 0 and good")} else {return println!("Stack is not empty")}

    //More optimal approach on memory usage
    let mut stack = std::collections::VecDeque::new(); //using VecDeque for better performance on push/pop operations from the Rust Std Library
    stack.reserve(string.len() / 2); // Reserve space in the VecDeque to avoid reallocations, assuming the string is balanced
    for c in string.chars() { // Iterate through each character in the string
        match c { // Match each character to the correct type of parenthesis
            '(' | '{' | '[' => stack.push_back(c), // Push opening brackets onto the stack
            ')' | '}' | ']' => { // For closing brackets
                let d = match stack.pop_back() { // Pop the last opening bracket from the stack
                    Some(d) => d, // If the stack is empty, this will return None
                    None => return false, // If the stack is empty, return false immediately
                };
                println!("{c} {d}");
                if !match (d, c) { // Check if the popped opening bracket matches the closing bracket and return based on that result
                    ('(', ')') | ('{', '}') | ('[', ']') => true, // this is the set of pairs that can return true
                    _ => false, // If they don't match, return false
                } {
                    return false; // if there is no beginning parenthesis this will return false here
                }
            }
            _ => (),
        }
    }


}
