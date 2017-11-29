use std::env;
use std::collections::HashMap;

type ArgumentList = HashMap<String, String>;

type Flags = Vec<String>;

/**
* Parser structure
*/
pub struct Parser{
    prefixes: Option<Vec<String>>
}

/**
* Parser implementation
*/
impl Parser {

    /**
    * Constructor
    *
    * @return Parser
    */
    pub fn new() -> Parser {
        Parser {
            prefixes: None
        }
    }

    /**
    * Sets the prefix for the arguments
    *
    * @param prefixes Vec<String>
    * @return Parser
    */
    pub fn strict_prefix(&mut self, prefixes: Vec<String>) -> &Self {
        self.prefixes = Some(prefixes);
        self
    }

    /**
    * Parses the given cmd arguments
    *
    * @return Touple(ArgumentList, Flags)
    */
    pub fn parse(&self) -> (ArgumentList, Flags) {
        let mut named_arguments: ArgumentList = ArgumentList::new();
        let mut flags: Flags = Flags::new();
        let mut argument_name: String = String::new();
        let mut i = 0;
        let argument_prefixes = match self.prefixes {
            Some(ref prefixes) => prefixes.clone(),
            None => vec!("-".to_owned(), "--".to_owned())
        };

        for (index, argument) in env::args().collect::<Vec<String>>().iter().enumerate() {
            for prefix in &argument_prefixes {
                if argument.starts_with(&prefix[..]) && argument_name != "" {
                    flags.push(argument_name.to_owned());
                    argument_name = argument[prefix.len()..argument.len()].to_owned();
                    i = index + 1;
                } else if argument.starts_with(&prefix[..]) {
                    argument_name = argument[prefix.len()..argument.len()].to_owned();
                    i = index + 1;
                } else if index == i && argument_name != "" {
                    named_arguments.insert(argument_name.to_owned(), argument.to_owned());
                    argument_name = "".to_owned();
                }
            }
        }

        if argument_name != "" {
            flags.push(argument_name.to_owned());
        }

        (named_arguments, flags)
    }
}