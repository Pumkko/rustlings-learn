// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this!
    /*let mut string_input = String::from(input);
    string_input.push_str(" world!");
    string_input */

    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let cars_offset = input.find("cars").unwrap_or(input.len());
    let cars_length_offset = cars_offset + "cars".len();

    let mut string_input = String::from(input);

    string_input.replace_range(cars_offset..cars_length_offset, "balloons");
    string_input
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
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
