fn main() {
    let a = b'(';
    let b = b'[';
    let c = b'{';
    let d = b')';
    let e = b']';
    let f = b'}';
    println!("{} {} {} {} {} {}", a, b, c, d, e, f);

    let string = "([}}])";

    
    // First pass at leetcode problem Valid Parentheses
    let bytes = string.as_bytes();
    println!("Bytes: {:?}", bytes);
    let mut stack: Vec<u8> = Vec::new();

    for(i, &item) in bytes.iter().enumerate() {

        if matches!(item as u8, b'{' | b'[' | b'(') {
            println!("Found '{item}'");
            stack.push(item);
        }

        if matches!(item, b'}' | b']' | b')') {
            if stack.len() >= 1 {
                let lastChar = stack[stack.len() - 1];
                println!("Found '{item}'");
                println!("Comparing '{lastChar}' with '{item}'");
                if (item).abs_diff(lastChar) <= 2 {
                    stack.pop();
                } else {
                    return println!("{} is not balanced", string);
                }
            } else {return println!("{} is not balanced", string)}
        }
    }
    if stack.len() == 0 {return println!("Stack is 0 and good")} else {return println!("Stack is not empty")}
    
    //More optimal approach on memory usage
    
    

}
