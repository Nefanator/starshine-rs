pub struct Command {

}

impl Command {
    pub fn new(args: &Vec<String>) -> Result<Command, &str> {
        if args.len() < 2 {
            return Err("invalid arguments; no arguments specified. Type -h for help");
        }

        if args.len() < 3 {
            return Err("invalid arguments; not enough arguments specified. Type -h for help");
        }

        unimplemented!()
    }
}

pub fn run(command: &Command) -> Result<(), &str> {
    unimplemented!()
}