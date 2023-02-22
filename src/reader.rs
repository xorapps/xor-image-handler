use crate::ImageType;
use camino::{Utf8Path, Utf8PathBuf};
use std::fmt;
use xor_errors::{XorError, XorResult};

#[cfg(feature = "full_smol")]
use async_fs::File;
#[cfg(feature = "full_smol")]
use futures_lite::{
    io::{BufReader, BufWriter},
    AsyncReadExt, AsyncWriteExt,
};

#[cfg(feature = "full_tokio")]
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};

#[cfg(feature = "base64")]
use base64::{engine::general_purpose, Engine as _};

#[derive(Debug, Default)]
pub struct ImageReader {
    files: Vec<Utf8PathBuf>,
    max_file_size: u64,
}

impl ImageReader {
    pub fn new() -> Self {
        ImageReader::default()
    }

    pub fn add_file_path(&mut self, file_path: &str) -> &mut Self {
        self.files.push(file_path.into());

        self
    }

    pub fn add_max_file_size(&mut self, max_file_size: u64) -> &mut Self {
        self.max_file_size = max_file_size;

        self
    }

    pub fn from_bytes(&mut self, size: usize) -> &mut Self {
        self.add_max_file_size(size as u64);

        self
    }

    pub fn from_kib(&mut self, size: usize) -> &mut Self {
        let size = size * 1024;

        self.add_max_file_size(size as u64);

        self
    }

    pub fn from_mib(&mut self, size: usize) -> &mut Self {
        let size = size * 1024 * 1024;

        self.add_max_file_size(size as u64);

        self
    }

    pub fn from_gib(&mut self, size: usize) -> &mut Self {
        let size = size * 1024 * 1024 * 1024;

        self.add_max_file_size(size as u64);

        self
    }

    pub async fn get_images(&self) -> XorResult<Vec<ImageWithMime>> {
        let mut outcome = Vec::<ImageWithMime>::new();

        for file_path in self.files.iter() {
            outcome.push(self.read_file(&file_path).await?);
        }

        Ok(outcome)
    }

    #[cfg(feature = "full_smol")]
    pub async fn read_file(&self, path: &Utf8Path) -> XorResult<ImageWithMime> {
        let file_stem = match path.file_stem() {
            Some(value) => value,
            None => {
                return Err(XorError::FilePathExt {
                    cause: "No File Stem or Invalid File Path".to_owned(),
                    path: path.to_string(),
                })
            }
        }
        .to_owned();

        let extension = match path.extension() {
            Some(value) => value,
            None => {
                return Err(XorError::FilePathExt {
                    cause: "No File Extension or Invalid File Extension".to_owned(),
                    path: path.to_string(),
                })
            }
        }
        .to_owned();

        let mime = ImageType::from_extension(&extension);

        let file = File::open(path).await?;

        let metadata = file.metadata().await?;

        if metadata.is_dir() {
            return Err(XorError::FilePath(path.to_string()));
        }

        if metadata.len() > self.max_file_size {
            return Err(XorError::FileSizeExceeded {
                capacity_allowed: self.max_file_size,
                size_encountered: metadata.len(),
            });
        }

        let mut file = BufReader::new(file);

        let mut buffer = [0u8; 1024];
        let mut container = Vec::<u8>::new();

        loop {
            let bytes_read = file.read(&mut buffer).await?;

            if bytes_read == 0 {
                break;
            }

            container.append(&mut buffer[..bytes_read].to_vec());
        }

        Ok(ImageWithMime {
            file_stem,
            extension,
            mime,
            bytes: container,
        })
    }

    #[cfg(feature = "full_tokio")]
    pub async fn read_file(&self, path: &Utf8Path) -> XorResult<ImageWithMime> {
        let file_stem = match path.file_stem() {
            Some(value) => value,
            None => {
                return Err(XorError::FilePathExt {
                    cause: "No File Stem or Invalid File Path".to_owned(),
                    path: path.to_string(),
                })
            }
        }
        .to_owned();

        let extension = match path.extension() {
            Some(value) => value,
            None => {
                return Err(XorError::FilePathExt {
                    cause: "No File Extension or Invalid File Extension".to_owned(),
                    path: path.to_string(),
                })
            }
        }
        .to_owned();

        let mime = ImageType::from_extension(&extension);

        let file = File::open(path).await?;

        let metadata = file.metadata().await?;

        if metadata.is_dir() {
            return Err(XorError::FilePath(path.to_string()));
        }

        if metadata.len() > self.max_file_size {
            return Err(XorError::FileSizeExceeded {
                capacity_allowed: self.max_file_size,
                size_encountered: metadata.len(),
            });
        }

        let mut file = BufReader::new(file);

        let mut buffer = [0u8; 1024];
        let mut container = Vec::<u8>::new();

        loop {
            let bytes_read = file.read(&mut buffer).await?;

            if bytes_read == 0 {
                break;
            }

            container.append(&mut buffer[..bytes_read].to_vec());
        }

        Ok(ImageWithMime {
            file_stem,
            extension,
            mime,
            bytes: container,
        })
    }

    #[cfg(feature = "full_smol")]
    pub async fn write_to_file(
        &self,
        file_stem: &str,
        file_extension: &str,
        mut bytes: &[u8],
    ) -> XorResult<Utf8PathBuf> {
        let mut file_name = Utf8PathBuf::new();
        file_name.set_file_name(file_stem);
        file_name.set_extension(file_extension);

        let file = File::create(&file_name).await?;
        let mut file = BufWriter::new(file);

        let mut buffer = [0u8; 1024];
        loop {
            let bytes_read = bytes.read(&mut buffer).await?;

            if bytes_read == 0 {
                break;
            }

            file.write(&buffer[..bytes_read]).await?;
        }

        file.flush().await?;

        Ok(file_name)
    }

    #[cfg(feature = "full_tokio")]
    pub async fn write_to_file(
        &self,
        file_stem: &str,
        file_extension: &str,
        bytes: &[u8],
    ) -> XorResult<Utf8PathBuf> {
        let mut file_name = Utf8PathBuf::new();
        file_name.set_file_name(file_stem);
        file_name.set_extension(file_extension);

        let file = File::create(&file_name).await?;
        let mut writer = BufWriter::new(file);

        writer.write_all(bytes).await?;

        Ok(file_name)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone)]
pub struct ImageWithMime {
    file_stem: String,
    extension: String,
    mime: ImageType,
    bytes: Vec<u8>,
}

impl ImageWithMime {
    pub fn new() -> Self {
        ImageWithMime::default()
    }

    pub fn file_stem(&self) -> &str {
        self.file_stem.as_str()
    }

    pub fn extension(&self) -> &str {
        self.extension.as_str()
    }

    pub fn mime(&self) -> ImageType {
        self.mime
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn add_extension(&mut self, extension: &str) -> &mut Self {
        self.extension = extension.to_owned();

        self
    }

    pub fn from_memory(&mut self, bytes: Vec<u8>) -> XorResult<&mut Self> {
        self.bytes = bytes;

        Ok(self)
    }

    pub fn sanity_check(&self, capacity: u64) -> XorResult<&Self> {
        if self.bytes.len() as u64 > capacity {
            return Err(XorError::FileSizeExceeded {
                capacity_allowed: capacity,
                size_encountered: self.bytes.len() as u64,
            });
        }

        Ok(self)
    }
}

impl fmt::Debug for ImageWithMime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_byte_size = self.bytes.len() as f32 / 1024f32 / 1024f32;
        let mut formatted_byte_size = formatted_byte_size.to_string();
        formatted_byte_size += "MiB";

        f.debug_struct("ImageWithMime")
            .field("file_stem", &self.file_stem)
            .field("extension", &self.extension)
            .field("mime", &self.mime)
            .field("bytes", &formatted_byte_size)
            .finish()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Encoding {
    #[cfg(feature = "base64")]
    Base64,
    #[cfg(feature = "hex")]
    Hex,
    #[cfg(feature = "lz4")]
    Lz4,
    #[cfg(all(feature = "lz4", feature = "base64"))]
    Lz4Base64,
    #[cfg(all(feature = "lz4", feature = "z85"))]
    Lz4Z85,
    #[cfg(feature = "lz4")]
    Lz4HC,
    #[cfg(all(feature = "lz4", feature = "base64"))]
    Lz4HCBase64,
    #[cfg(all(feature = "lz4", feature = "z85"))]
    Lz4HCZ85,
}

impl Encoding {
    //TODO Support Lz4HC
    pub fn encode_string(&self, data: &[u8]) -> XorResult<String> {
        let encoded = match self {
            #[cfg(feature = "base64")]
            Self::Base64 => Encoding::base64encode(data),
            #[cfg(feature = "hex")]
            Self::Hex => hex::encode(data),
            #[cfg(feature = "lz4")]
            Self::Lz4 => return Err(XorError::UnsupportedStringEncoding("Lz4")),
            #[cfg(all(feature = "lz4", feature = "base64"))]
            Self::Lz4Base64 => {
                let compressed = lz4_flex::compress(data);
                Encoding::base64encode(&compressed)
            }
            #[cfg(all(feature = "lz4", feature = "z85"))]
            Self::Lz4Z85 => {
                let compressed = lz4_flex::compress(data);

                z85::encode(&compressed)
            }
            #[cfg(feature = "lz4")]
            Self::Lz4HC => return Err(XorError::UnsupportedFormat("Lz4HC")),
            #[cfg(all(feature = "lz4", feature = "base64"))]
            Self::Lz4HCBase64 => return Err(XorError::UnsupportedFormat("Lz4HCBase64")),
            #[cfg(all(feature = "lz4", feature = "z85"))]
            Self::Lz4HCZ85 => return Err(XorError::UnsupportedFormat("Lz4HCZ85")),
        };

        Ok(encoded)
    }

    //TODO Support Lz4HC
    #[allow(unused_variables)]
    pub fn encode_binary(&self, data: &[u8]) -> XorResult<Vec<u8>> {
        match self {
            #[cfg(feature = "base64")]
            Self::Base64 => return Err(XorError::UnsupportedBinaryEncoding("Base64")),
            #[cfg(feature = "hex")]
            Self::Hex => return Err(XorError::UnsupportedBinaryEncoding("Hex")),
            #[cfg(feature = "lz4")]
            Self::Lz4 => Ok(lz4_flex::compress_prepend_size(data)),
            #[cfg(all(feature = "lz4", feature = "base64"))]
            Self::Lz4Base64 => return Err(XorError::UnsupportedBinaryEncoding("Lz4Base64")),
            #[cfg(all(feature = "lz4", feature = "z85"))]
            Self::Lz4Z85 => return Err(XorError::UnsupportedBinaryEncoding("Lz4Z85")),
            #[cfg(feature = "lz4")]
            Self::Lz4HC => return Err(XorError::UnsupportedFormat("Lz4HC")),
            #[cfg(all(feature = "lz4", feature = "base64"))]
            Self::Lz4HCBase64 => return Err(XorError::UnsupportedFormat("Lz4HCBase64")),
            #[cfg(all(feature = "lz4", feature = "z85"))]
            Self::Lz4HCZ85 => return Err(XorError::UnsupportedFormat("Lz4HCZ85")),
        }
    }

    #[cfg(feature = "base64")]
    pub fn base64encode(data: &[u8]) -> String {
        general_purpose::STANDARD_NO_PAD.encode(data)
    }

    #[cfg(feature = "base64")]
    pub fn base64decode(data: &str) -> XorResult<Vec<u8>> {
        Ok(general_purpose::STANDARD_NO_PAD.decode(data)?)
    }

    //TODO Support Lz4HC
    pub fn decode(&self, data: &str) -> XorResult<Vec<u8>> {
        let decoded = match self {
            #[cfg(feature = "base64")]
            Self::Base64 => general_purpose::STANDARD_NO_PAD.decode(data)?,
            #[cfg(feature = "hex")]
            Self::Hex => hex::decode(data)?,
            #[cfg(feature = "lz4")]
            Self::Lz4 => return Err(XorError::UnsupportedDecodeString("Lz4")),
            #[cfg(all(feature = "lz4", feature = "base64"))]
            Self::Lz4Base64 => {
                let decoded = general_purpose::STANDARD_NO_PAD.decode(data)?;

                lz4_flex::decompress_size_prepended(&decoded)?
            }
            #[cfg(all(feature = "lz4", feature = "z85"))]
            Self::Lz4Z85 => {
                let decoded = z85::decode(&data)?;

                lz4_flex::decompress_size_prepended(&decoded)?
            }
            #[cfg(feature = "lz4")]
            Self::Lz4HC => return Err(XorError::UnsupportedFormat("Lz4HC")),
            #[cfg(all(feature = "lz4", feature = "base64"))]
            Self::Lz4HCBase64 => return Err(XorError::UnsupportedFormat("Lz4HCBase64")),
            #[cfg(all(feature = "lz4", feature = "z85"))]
            Self::Lz4HCZ85 => return Err(XorError::UnsupportedFormat("Lz4HCZ85")),
        };

        Ok(decoded)
    }

    #[allow(unused_variables)]
    pub fn decompress(&self, data: &[u8]) -> XorResult<Vec<u8>> {
        let decompressed = match self {
            #[cfg(feature = "base64")]
            Self::Base64 => return Err(XorError::UnsupportedDecodeBinary("Base64")),
            #[cfg(feature = "hex")]
            Self::Hex => return Err(XorError::UnsupportedDecodeBinary("Hex")),
            #[cfg(feature = "lz4")]
            Self::Lz4 => Ok(lz4_flex::decompress_size_prepended(data)?),
            #[cfg(all(feature = "lz4", feature = "base64"))]
            Self::Lz4Base64 => return Err(XorError::UnsupportedDecodeBinary("Lz4Base64")),
            #[cfg(all(feature = "lz4", feature = "z85"))]
            Self::Lz4Z85 => return Err(XorError::UnsupportedDecodeBinary("Lz4Z85")),
            #[cfg(feature = "lz4")]
            Self::Lz4HC => return Err(XorError::UnsupportedFormat("Lz4HC")),
            #[cfg(all(feature = "lz4", feature = "base64"))]
            Self::Lz4HCBase64 => return Err(XorError::UnsupportedFormat("Lz4HCBase64")),
            #[cfg(all(feature = "lz4", feature = "z85"))]
            Self::Lz4HCZ85 => return Err(XorError::UnsupportedFormat("Lz4HCZ85")),
        };

        #[allow(unreachable_code)]
        // This code is unreachable if only the default features are selected since `hex` already has a `return` type
        Ok(decompressed)
    }
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::Hex
    }
}
