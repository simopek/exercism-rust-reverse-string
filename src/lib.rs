pub fn reverse(input: &str) -> String {

    let mut reversed_chars = Vec::with_capacity(input.chars().count());

    for c in input.chars() {
        reversed_chars.insert(0, c);
    }

    reversed_chars.iter().collect::<String>()
}
