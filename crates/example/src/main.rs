fn main() {
    let regex = regex::Regex::new(r"\$__children(?::([^_]+))?__\$").unwrap();
    let template = r"int $__children:a__$ lol $__children:b__$";

    // Use captures_iter to find all matches
    for captures in regex.captures_iter(template) {
        // Capture the entire placeholder
        if let Some(full_match) = captures.get(0) {
            println!("Full match: {}", full_match.as_str());
        }

        // Capture the part after the colon (if it exists)
        if let Some(part_after_colon) = captures.get(1) {
            println!("Captured: {}", part_after_colon.as_str());
        }
    }

    let text = "main";
    let replaced = regex.replace_all(template, text);
    println!("{}", replaced);
}