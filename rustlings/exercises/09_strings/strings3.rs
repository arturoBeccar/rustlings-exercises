fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    // V0
    //input.trim()
    // V1
    let mut start = 0;
    let mut end = input.len();

    // Find the index of the first non-whitespace character
    for (i, c) in input.char_indices() {
        if !c.is_whitespace() {
            start = i;
            break;
        }
    }

    // Find the index of the last non-whitespace character
    for (i, c) in input.char_indices().rev() {
        if !c.is_whitespace() {
            end = i + c.len_utf8(); // Account for multi-byte characters
            break;
        }
    }

    // Return the slice of the string without leading/trailing whitespace
    &input[start..end]
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    // V0
    // input.replace("cars", "balloons")
    // V1
    let target = "cars";
    let replacement = "balloons";
    let mut result = String::new();
    
    let mut i = 0;
    while i < input.len() {
        if input[i..].starts_with(target) {
            // If we find "cars", append "balloons" to result
            result.push_str(replacement);
            i += target.len(); // Move the index past the matched word
        } else {
            // Otherwise, append the current character to result
            result.push(input[i..].chars().next().unwrap());
            i += input[i..].chars().next().unwrap().len_utf8(); // Move the index forward by the length of the current character
        }
    }
    
    result
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
