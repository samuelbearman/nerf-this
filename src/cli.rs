use clap::{Parser};

#[derive(Parser, Debug)]
pub struct Args {
    pub url: String,
}

// pub fn parse_arguments() -> Args {
//     let matches = App::new("nerfthis")
//         .version("0.0.1")
//         .arg
// }