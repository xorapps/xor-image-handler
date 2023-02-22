use xor_errors::{XorError, XorResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImageType {
    Svg,
    Png,
    Jpeg,
    Gif,
    Avif,
    WebP,
    Unsupported,
}

impl ImageType {
    pub fn to_html_ext(&self) -> XorResult<&str> {
        let mime = match self {
            Self::Svg => "data:image/svg+xml",
            Self::Png => "data:image/png",
            Self::Jpeg => "data:image/jpeg",
            Self::Gif => "data:image/gif",
            Self::Avif => "data:image/avif",
            Self::WebP => "data:image/webp",
            _ => return Err(XorError::UnsupportedImageFormat),
        };

        Ok(mime)
    }

    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "svg" => Self::Svg,
            "png" => Self::Png,
            "jpeg" => Self::Jpeg,
            "gif" => Self::Gif,
            "avif" => Self::Avif,
            "webp" => Self::WebP,
            _ => Self::Unsupported,
        }
    }
}

impl Default for ImageType {
    fn default() -> Self {
        ImageType::Svg
    }
}
