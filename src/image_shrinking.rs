use imagelib::{
    bmp, gif, guess_format, hdr, ico, jpeg, load_from_memory_with_format, png, pnm, tiff, webp,
    DynamicImage, ImageDecoder, ImageError, ImageFormat,
};
use std::borrow::Cow;
use std::io::Cursor;

fn decode_dims<D: ImageDecoder>(mut decoder: D) -> Result<(u32, u32), ImageError> {
    decoder.dimensions()
}

fn shrink_inner(data: &[u8], max_dim: u32) -> Result<Cow<[u8]>, ImageError> {
    let format = guess_format(data)?;

    let cursor = Cursor::new(data);
    let (width, height) = match format {
        ImageFormat::PNG => decode_dims(png::PNGDecoder::new(cursor))?,
        ImageFormat::JPEG => decode_dims(jpeg::JPEGDecoder::new(cursor))?,
        ImageFormat::GIF => decode_dims(gif::Decoder::new(cursor))?,
        ImageFormat::WEBP => decode_dims(webp::WebpDecoder::new(cursor))?,
        ImageFormat::TIFF => decode_dims(tiff::TIFFDecoder::new(cursor)?)?,
        ImageFormat::BMP => decode_dims(bmp::BMPDecoder::new(cursor))?,
        ImageFormat::ICO => decode_dims(ico::ICODecoder::new(cursor)?)?,
        ImageFormat::HDR => decode_dims(hdr::HDRAdapter::new(cursor)?)?,
        ImageFormat::PNM => decode_dims(pnm::PNMDecoder::new(cursor)?)?,
        ImageFormat::TGA => return Ok(data.into()), // TGA doesn't have a Header
    };

    let is_too_large = width > max_dim || height > max_dim;
    if is_too_large || format == ImageFormat::BMP {
        let mut image = load_from_memory_with_format(data, format)?;
        if is_too_large {
            image = image.thumbnail(max_dim, max_dim);
        }
        let mut data = Vec::new();
        let ((width, height), image_data) = match &image {
            DynamicImage::ImageLuma8(x) => (x.dimensions(), x.as_ref()),
            DynamicImage::ImageLumaA8(x) => (x.dimensions(), x.as_ref()),
            DynamicImage::ImageRgb8(x) => (x.dimensions(), x.as_ref()),
            DynamicImage::ImageRgba8(x) => (x.dimensions(), x.as_ref()),
            DynamicImage::ImageBgr8(x) => (x.dimensions(), x.as_ref()),
            DynamicImage::ImageBgra8(x) => (x.dimensions(), x.as_ref()),
        };
        png::PNGEncoder::new(&mut data).encode(image_data, width, height, image.color())?;
        Ok(data.into())
    } else {
        Ok(data.into())
    }
}

pub fn shrink(data: &[u8], max_dim: u32) -> Cow<[u8]> {
    shrink_inner(data, max_dim).unwrap_or_else(|_| data.into())
}
