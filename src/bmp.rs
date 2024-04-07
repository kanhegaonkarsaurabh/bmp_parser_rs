use std::{
    fs::File,
    io::{Error, Read},
};
// ref for BMP spec: https://upload.wikimedia.org/wikipedia/commons/7/75/BMPfileFormat.svg

pub struct BMPFileHeader {
    // (2 bytes) signature of the bmp file
    pub signature: u16,
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

    // (4 bytes each) X&Y pixels per meter or resolution
    // of the image
    x_resolution: i32,
    y_resolution: i32,

    // (4 bytes) number of colors in the color_table
    num_colors: u32,
    // (4 bytes) important color count
    imp_colors: u32,
}

pub struct BMPFile {
    pub header: BMPFileHeader,
    // dib_header: BMPDIBHeader,
    // img_data: Vec<Vec<u8>>,
}

impl BMPFile {
    pub fn new(bmp_filepath: &str) -> Result<Self, Error> {
        let mut bmp_file = File::open(bmp_filepath)?;

        // first we get the signature bits in header and
        // validate that the signature is correct
        let mut bmp_signature_bytes: Vec<u8> = vec![0; 2];
        bmp_file.read_exact(&mut bmp_signature_bytes)?;

        if !BMPFile::validate_header_signature(&bmp_signature_bytes) {
            panic!(
                "{:?} is an invalid BMP file due to invalid BMP signature: [{:#x}, {:#x}]",
                bmp_filepath, bmp_signature_bytes[0], bmp_signature_bytes[1]
            );
        }

        let bmp_signature: u16 =
            ((bmp_signature_bytes[0] as u16) << 8) | bmp_signature_bytes[1] as u16;

        Ok(Self {
            header: BMPFileHeader {
                signature: bmp_signature,
            },
        })
    }
    fn validate_header_signature(signature: &[u8]) -> bool {
        // 0x42 = B
        // Ox4D = M
        // 'BM' is the signature for common bmp files
        signature[0] == 0x42 && signature[1] == 0x4D
    }
}
