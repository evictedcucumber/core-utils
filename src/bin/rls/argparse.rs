use std::env::Args;

pub enum ArgumentType {
    Help,
    Version,
}

pub fn parse_args(argc: usize, argv: &Args) -> Vec<ArgumentType> {
    let mut args: Vec<ArgumentType> = Vec::new();
    args.push(ArgumentType::Help);
    args.push(ArgumentType::Version);
    args
}
