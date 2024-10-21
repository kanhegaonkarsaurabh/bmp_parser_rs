use std::env;
use std::io::Error;

mod bmp;
use bmp::BMPFile;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let maybe_bmp = BMPFile::new(args[1].clone());

    match maybe_bmp {
        Ok(bmp) => {
            println!("signature: {:#x}", bmp.header.signature);
        }
        Err(error) => panic!("{:?}", error),
    }

    Ok(())
}
