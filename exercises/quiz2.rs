// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

/* 
Not super fan of into() here, i understand that it works from context, eg 
let f: f64 = 30.into(); 
because f was declared as a f64, then into will turn 30 into an f64. 

then for this quizz : ("hello".into(), Command::Uppercase)
"hello" will turn into the type declared in the tuple, which is Vec<String,Command> so into() will turn "hello" into a String.

I find that to be waaaaaay too unclear, I understand why they would to that in context of the quizz
but still i find to_owned() to be much much clearer. Because it's what we are doing
we have a borrewed string &str and we want to own it now to update it. 
*/
mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            let updated_string = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_owned(),
                Command::Append(size) => {
                    let mut owned_string = string.to_owned();
                    owned_string.push_str(&"bar".repeat(*size));
                    owned_string
                }
            };
            output.push(updated_string);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
