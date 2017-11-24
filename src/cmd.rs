pub mod parser {
    use std::env;
    use std::collections::HashMap;

    type ArgumentList = HashMap<String, String>;

    type Flags = Vec<String>;

    pub fn parse_arguments() -> (ArgumentList, Flags) {
        let mut named_arguments: ArgumentList = ArgumentList::new();
        let mut flags: Flags = Flags::new();
        let mut argument_name: String = String::new();
        let mut i = 0;

        for (index, argument) in env::args().collect::<Vec<_>>().iter().enumerate() {
            if (argument.starts_with("--") || argument.starts_with("-")) && argument_name != "" {
                flags.push(argument_name.to_owned());
                if argument.starts_with("--") {
                    flags.push(argument[2..argument.len()].to_owned());
                } else if argument.starts_with("-") {
                    flags.push(argument[1..argument.len()].to_owned());
                }
                argument_name = "".to_owned();
            } else if argument.starts_with("--") {
                argument_name = argument[2..argument.len()].to_owned();
                i = index + 1;
            } else if argument.starts_with("-") {
                argument_name = argument[1..argument.len()].to_owned();
                i = index + 1;
            } else if index == i && argument_name != "" {
                named_arguments.insert(argument_name.to_owned(), argument.to_owned());
                argument_name = "".to_owned();
            }
        }

        if argument_name != "" {
            flags.push(argument_name.to_owned());
        }

        (named_arguments, flags)
    }
}