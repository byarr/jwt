use std::env::args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let args: Vec<_> = args().collect();
    jwt::decode_jwt(args.get(1).expect("Not enough args"))?;

    println!("ok");
    Ok(())
}
