use anyhow::{anyhow, Result};
use std::env;

pub fn parse_args() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(anyhow!("Usage: {} <transactions.csv>", args[0]));
    }
    Ok(args[1].clone())
}
