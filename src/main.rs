use std::env;
use std::io::Error;

mod bmp;
use bmp::BMPFile;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let maybe_bmp = BMPFile::new(&args[1]);

    match maybe_bmp {
        Ok(bmp) => {
            println!(
                "signature: [{:#x}, {:#x}]",
                bmp.header.signature[0], bmp.header.signature[1]
            );
        }
        Err(error) => panic!("{:?}", error),
    }

    Ok(())
}
