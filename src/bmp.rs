use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};
// ref for BMP spec: https://upload.wikimedia.org/wikipedia/commons/7/75/BMPfileFormat.svg

pub struct BMPFileHeader<'a> {
    // (2 bytes) signature of the bmp file
    pub signature: &'a [u8; 2],
    // (4 bytes) size of the bmp file
    // size: u32,
    // (2 bytes) reserved byte 1 for the spec
    // reserved1: u16,
    // reserved2: u16,
    // (4 byte) offset from the header where img data is located
    // offset: u32,
}

struct BMPDIBHeader {
    // (4 bytes) overall size of *this* DIB header
    size: u32,
    // (4 bytes) width of the image
    img_width: u32,
    // (4 bytes) height of the image
    img_height: u32,
    // (2 bytes) number of colored planes
    planes: u16,
    // (2 bytes) number of bits-per-pixel
    bits_per_px: u16,
    // (4 bytes) compression type used by image
    compression_type: u32,
    // (4 bytes) size of the image in bytes
    img_size: u32,

    //
    // of the image
    x_resolution: i32,
    y_resolution: i32,

    // (4 bytes) number of colors in the color_table
    num_colors: u32,
    // (4 bytes) important color count
    imp_colors: u32,
}

pub struct BMPFile<'a> {
    pub header: BMPFileHeader<'a>,
    // dib_header: BMPDIBHeader,
    // img_data: Vec<Vec<u8>>,
}

impl<'a> BMPFile<'a> {
    pub fn validate_file_signature(mut bmp_file: &'a File) -> Result<&'static [u8; 2], Error> {
        // validate that the signature is correct
        let mut bmp_signature_bytes: Vec<u8> = vec![0; 2];
        bmp_file.read_exact(&mut bmp_signature_bytes)?;

        if &bmp_signature_bytes[..2] == b"BM" {
            // tried returning OK(&bmp_signature_bytes[..2]) here but somehow
            // couldn't type constrain it to be &[u8; 2] as it was returning &[u8]
            // instead
            return Ok(b"BM");
        }

        // failure of signature
        return Err(Error::new(
            ErrorKind::InvalidData,
            "invalid bmp signature header",
        ));
    }

    pub fn parse(bmp_file: &'a File, bmp_signature: &'static [u8; 2]) -> Result<Self, Error> {
        Ok(Self {
            header: BMPFileHeader {
                signature: bmp_signature,
            },
        })
    }

    // NOTE: lifetime of the BMPFile struct will live as long as the lifetime
    // of the incoming file bmp_file such that the BMPFile shape lives as long
    // as the data source (file here)
    pub fn new(bmp_file: &'a File) -> Result<Self, Error> {
        match Self::validate_file_signature(bmp_file) {
            Ok(bmp_signature) => Self::parse(bmp_file, bmp_signature),
            Err(error) => Err(error),
        }
    }
}
