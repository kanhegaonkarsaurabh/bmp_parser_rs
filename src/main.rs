use std::io::Error;

mod bmp;
use bmp::BMPFile;

fn main() -> Result<(), Error> {
    let maybe_bmp = BMPFile::new("monochrome_test.bmp");

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
