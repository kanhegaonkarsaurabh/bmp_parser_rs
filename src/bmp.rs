use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
    rc::Weak,
};
// ref for BMP spec: https://upload.wikimedia.org/wikipedia/commons/7/75/BMPfileFormat.svg

pub struct BMPFileHeader<'a> {
    // (2 bytes) signature of the bmp file
    pub signature: &'a [u8; 2],
    // (4 bytes) size of the bmp file
    size: u32,
    // (2 bytes) reserved byte 1 for the spec
    reserved1: u16,
    // (2 bytes) reserved byte 2 for the spec
    reserved2: u16,
    // (4 byte) offset from the header where img data is located
    offset: u32,
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
    pub fn validate_file_signature(mut bmp_file: &'a File) -> Result<&'a [u8; 2], Error> {
        // validate that the signature is correct
        let mut bmp_signature_bytes = vec![0; 2];

        // NOTE: the buffer to read into is being passed in as a mutable ref &mut as we're
        // saying that read_exact(..) can mutate the data that bmp_signature_bytes
        // references, but the actual referenece cannot be changed and is immutable.
        // definitely confusing so read it again :)
        bmp_file.read_exact(&mut bmp_signature_bytes)?;

        if &bmp_signature_bytes[..2] == b"BM" {
            // tried returning OK(&bmp_signature_bytes[..2]) here but somehow
            // couldn't type constrain it to be &[u8; 2] as it was returning &[u8]
            // instead

            // NOTE: also, b"BM" has type &'static [u8; 2] and a lifetime of static
            // which is a super set in terms of lifetime 'a since static outlasts
            // everything and also why we can return it here even though fn signature
            // expects &'a [u8; 2]
            return Ok(b"BM");
        }

        // failure of signature
        return Err(Error::new(
            ErrorKind::InvalidData,
            "invalid bmp signature header",
        ));
    }

    pub fn parse(mut bmp_file: &'a File, bmp_signature: &'a [u8; 2]) -> Result<Self, Error> {
        // 4 bytes file size, 2+2 for reserved bytes and then 4 bytes for the file offset
        let mut bmp_file_header = vec![0; 12];
        bmp_file.read_exact(&mut bmp_file_header)?;

        let file_size = u32::from_le_bytes(bmp_file_header[..4].try_into().unwrap());
        let reserved_one = u16::from_le_bytes(bmp_file_header[4..6].try_into().unwrap());
        let reserved_two = u16::from_le_bytes(bmp_file_header[6..8].try_into().unwrap());
        let offset_to_img_data = u32::from_le_bytes(bmp_file_header[8..].try_into().unwrap());

        Ok(Self {
            header: BMPFileHeader {
                signature: bmp_signature,
                size: file_size,
                reserved1: reserved_one,
                reserved2: reserved_two,
                offset: offset_to_img_data,
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

#[test]
fn test_bmp_file_header() -> Result<(), Error> {
    let bmp_file = File::open("./monochrome_test.bmp")?;
    let parsed_bmp = BMPFile::new(&bmp_file)?;
    assert_eq!(parsed_bmp.header.signature, b"BM");

    // -- validated the correct values via `hexdump -C monochrome_test.bmp`
    assert_eq!(parsed_bmp.header.size, 638);

    assert_eq!(parsed_bmp.header.reserved1, 0);
    assert_eq!(parsed_bmp.header.reserved2, 0);

    Ok(assert_eq!(parsed_bmp.header.offset, 62))
}
