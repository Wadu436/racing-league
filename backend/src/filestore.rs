use std::path::{self, PathBuf};

use color_eyre::Result;
use hex::ToHex;

use kv::Bucket;
use mime_guess::Mime;
use sha3::{Digest, Sha3_256};

use tower_http::services::ServeDir;

use bytes::Bytes;

pub struct FileStore {
    directory: PathBuf,
    bucket: Bucket<'static, String, String>,
    transformers: Vec<Transformer>,
}

pub struct File {
    pub bytes: Bytes,
    pub mime_type: Mime,
}

pub type Transformer = Box<dyn Fn(&File) -> Option<File> + Send + Sync>;

impl FileStore {
    pub fn new<P: AsRef<path::Path>>(directory: P) -> Result<Self> {
        // Create directory of it doesn't exist yet
        let cfg = kv::Config::new(directory.as_ref().join("file_hashes_kv"));
        let store = kv::Store::new(cfg)?;
        let bucket = store.bucket::<String, String>(Some("hashes"))?;

        let directory = directory.as_ref().to_owned();

        std::fs::create_dir_all(directory.join("files"))?;

        Ok(Self {
            directory,
            bucket,
            transformers: Vec::new(),
        })
    }

    pub fn register_transformer(&mut self, transformer: Transformer) {
        self.transformers.push(transformer);
    }

    pub async fn upload(&self, mime_type: Mime, bytes: bytes::Bytes) -> Result<String> {
        let mut hasher = Sha3_256::new();

        hasher.update(&bytes);

        let hash = hasher.finalize().encode_hex();

        let filename = if let Some(filename) = self.bucket.get(&hash)? {
            // File already exists
            filename
        } else {
            // File does not exist yet
            // Run transformers

            let mut file = File { bytes, mime_type };

            for transform in self.transformers.iter() {
                if let Some(new_file) = transform(&file) {
                    file = new_file;
                }
            }

            let extension = mime_guess::get_mime_extensions(&file.mime_type)
                .and_then(|extensions| extensions.first());

            let filename = if let Some(extension) = extension {
                format!("{}.{}", uuid::Uuid::new_v4().simple(), extension)
            } else {
                uuid::Uuid::new_v4().simple().to_string()
            };

            let path = self.directory.join("files").join(&filename);

            tokio::fs::write(&path, &file.bytes).await?;

            self.bucket.set(&hash, &filename)?;
            self.bucket.flush_async().await?;

            filename
        };

        Ok(filename)
    }

    pub fn service(&self) -> ServeDir {
        ServeDir::new(self.directory.join("files")).precompressed_br()
    }
}
