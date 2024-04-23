// Taken from https://github.com/ChurchTao/clipboard-rs/blob/master/src/common.rs
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageFormat};
use std::io::Cursor;
pub struct RustImageData {
    width: u32,
    height: u32,
    data: Option<DynamicImage>,
}

/// 此处的 RustImageBuffer 已经是带有图片格式的字节流，例如 png,jpeg;
pub struct RustImageBuffer(Vec<u8>);
// TODO: Learn more about error, merge this with crate::error::Error
pub type ImageResult<T> =
    std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub trait RustImage: Sized {
    /// create an empty image
    fn empty() -> Self;

    fn is_empty(&self) -> bool;

    /// Read image from file path
    fn from_path(path: &str) -> ImageResult<Self>;

    /// Create a new image from a byte slice
    fn from_bytes(bytes: &[u8]) -> ImageResult<Self>;

    fn from_dynamic_image(image: DynamicImage) -> Self;

    /// width and height
    fn get_size(&self) -> (u32, u32);

    /// Scale this image down to fit within a specific size.
    /// Returns a new image. The image's aspect ratio is preserved.
    /// The image is scaled to the maximum possible size that fits
    /// within the bounds specified by `nwidth` and `nheight`.
    ///
    /// This method uses a fast integer algorithm where each source
    /// pixel contributes to exactly one target pixel.
    /// May give aliasing artifacts if new size is close to old size.
    fn thumbnail(&self, width: u32, height: u32) -> ImageResult<Self>;

    /// en: Adjust the size of the image without retaining the aspect ratio
    /// zh: 调整图片大小，不保留长宽比
    fn resize(&self, width: u32, height: u32, filter: FilterType) -> ImageResult<Self>;

    fn to_jpeg(&self) -> ImageResult<RustImageBuffer>;

    /// en: Convert to png format, the returned image is a new image, and the data itself will not be modified
    /// zh: 转为 png 格式,返回的为新的图片，本身数据不会修改
    fn to_png(&self) -> ImageResult<RustImageBuffer>;

    fn to_bitmap(&self) -> ImageResult<RustImageBuffer>;

    fn save_to_path(&self, path: &str) -> ImageResult<()>;
}

macro_rules! image_to_format {
    ($name:ident, $format:expr) => {
        fn $name(&self) -> ImageResult<RustImageBuffer> {
            match &self.data {
                Some(image) => {
                    let mut bytes: Vec<u8> = Vec::new();
                    image.write_to(&mut Cursor::new(&mut bytes), $format)?;
                    Ok(RustImageBuffer(bytes))
                }
                None => Err("image is empty".into()),
            }
        }
    };
}

impl RustImage for RustImageData {
    fn empty() -> Self {
        RustImageData {
            width: 0,
            height: 0,
            data: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_none()
    }

    fn from_path(path: &str) -> ImageResult<Self> {
        let image = image::open(path)?;
        let (width, height) = image.dimensions();
        Ok(RustImageData {
            width,
            height,
            data: Some(image),
        })
    }

    fn from_bytes(bytes: &[u8]) -> ImageResult<Self> {
        let image = image::load_from_memory(bytes)?;
        let (width, height) = image.dimensions();
        Ok(RustImageData {
            width,
            height,
            data: Some(image),
        })
    }

    fn from_dynamic_image(image: DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        RustImageData {
            width,
            height,
            data: Some(image),
        }
    }

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn thumbnail(&self, width: u32, height: u32) -> ImageResult<Self> {
        match &self.data {
            Some(image) => {
                let resized = image.thumbnail(width, height);
                Ok(RustImageData {
                    width: resized.width(),
                    height: resized.height(),
                    data: Some(resized),
                })
            }
            None => Err("image is empty".into()),
        }
    }

    fn resize(&self, width: u32, height: u32, filter: FilterType) -> ImageResult<Self> {
        match &self.data {
            Some(image) => {
                let resized = image.resize_exact(width, height, filter);
                Ok(RustImageData {
                    width: resized.width(),
                    height: resized.height(),
                    data: Some(resized),
                })
            }
            None => Err("image is empty".into()),
        }
    }

    image_to_format!(to_jpeg, ImageFormat::Jpeg);

    image_to_format!(to_png, ImageFormat::Png);

    image_to_format!(to_bitmap, ImageFormat::Bmp);

    fn save_to_path(&self, path: &str) -> ImageResult<()> {
        match &self.data {
            Some(image) => {
                image.save(path)?;
                Ok(())
            }
            None => Err("image is empty".into()),
        }
    }
}

impl RustImageBuffer {
    pub fn get_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn save_to_path(&self, path: &str) -> ImageResult<()> {
        std::fs::write(path, &self.0)?;
        Ok(())
    }
}
