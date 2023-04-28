pub mod common {
    /// Parse command line arguments.
    pub fn parse_args() -> Vec<String> {
        let mut args = std::env::args();
        args.next(); // Skip the first argument, which is the program name.
        args.collect() // Return a Vec<String> of the arguments.
    }
}