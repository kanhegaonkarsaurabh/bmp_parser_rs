use std::env;
use std::fs::File;
use std::io::Error;

mod bmp;
use bmp::BMPFile;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let bmp_file = File::open(&args[1])?;
    let maybe_bmp = BMPFile::new(&bmp_file);

    // match maybe_bmp {
    //     Ok(bmp) => {
    //         println!("signature: {:#x}", bmp.header.signature);
    //     }
    //     Err(error) => panic!("{:?}", error),
    // }

    Ok(())
}
