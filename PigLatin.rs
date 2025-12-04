fn main() {
    let s = String::from("Hello  what is this apple");

    let vowels = String::from("aeiouAEIOU");

    let mut result: Vec<String> = Vec::new();

    for word in s.split_whitespace() {
        let mut chars = word.chars();

        if let Some(first) = chars.next() {
            if vowels.contains(first) {
                result.push(format!("{word}hay"));
            } else {
                let res: String = chars.collect();
                result.push(format!("{res}{first}ay"));
            }
        }
    }

    let result = result.join(" ");

    println!("{result}");
}
