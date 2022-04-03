use clap::{Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {

    /// Github repo url Ex. https://github.com/samuelbearman/nerf-this
    pub url: String,
    
    /// Max number of github api fetches
    #[clap(short, long, default_value_t = 1)]
    pub fetch_count: u32,
}