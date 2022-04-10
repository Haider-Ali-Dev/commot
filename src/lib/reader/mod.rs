use std::io::{BufReader, Read};
use std::fs::{File, metadata};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

use crate::errors::CommotError;

/// File Reader

#[derive(Clone, Debug)]
pub struct Reader {
    pub path: PathBuf,
}

/// `FileSizeType`
/// This enum has all the size types which are storing the size inside them.
#[derive(Clone, Debug)]
pub enum FileSizeType {
    Bytes(u64),
    KiloBytes(u64),
    MegaBytes(u64),
    GigaBytes(u64)
}


#[derive(Clone, Debug)]
pub struct FileMetadata {
    pub size: FileSize,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub accessed: DateTime<Utc>,
}
impl FileMetadata {
    pub fn new(
        size: FileSize,
        created: DateTime<Utc>,
        modified: DateTime<Utc>,
        accessed: DateTime<Utc>


    ) -> Self {
        Self {
            size,
            created,
            modified,
            accessed
        }
    }
}

impl FileSizeType {
    pub fn new(bytes: u64) -> Self {
        let kb = bytes / 1000;
        let mb = kb / 1000;
        let gb = mb / 1000;
        if bytes < 1000 {
            FileSizeType::Bytes(bytes)
        } else if bytes >= 1000 {
            FileSizeType::KiloBytes(kb)
        } else if kb >= 1000 {
            FileSizeType::MegaBytes(mb)
        } else {
            FileSizeType::GigaBytes(gb)
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Self::Bytes(v) => format!("{} byte(s)", v),
            Self::KiloBytes(v) => format!("{} kilobyte(s)", v),
            Self::MegaBytes(v) => format!("{} megabyte(s)", v),
            Self::GigaBytes(v) => format!("{} gigabyte(s)", v)
        }

    }

    pub fn into_inner(&self) -> u64 {
        match &self {
            FileSizeType::Bytes(v) => v.to_owned(),
            FileSizeType::KiloBytes(v) => v.to_owned(),
            FileSizeType::MegaBytes(v) => v.to_owned(),
            FileSizeType::GigaBytes(v) => v.to_owned(),
        }
    }

    
    
}
#[derive(Clone, Debug)]
pub struct FileSize(FileSizeType);

impl FileSize {
    pub fn new(size: FileSizeType) -> Self {
        Self(size)
    }

   pub fn into_inner(&self) -> FileSizeType {
       self.0.clone()
   }
}

pub struct FileData(String);
impl FileData {
    pub fn new(data: String) -> Self {
        Self(data)
    }

    pub fn into_inner(&self) -> String {
        self.0.to_owned()
    }
}
/// Implementing methods on the reader like "read", "file_size", etc.
impl Reader {
    pub fn new(path: String) -> Self{ 
        Self {path: path.into()}
    }
    pub fn read(&self) -> Result<FileData, CommotError> {
        let file = File::open(&self.path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        Ok(FileData::new(buffer))
    }

    
    pub fn size(&self) -> Result<FileSize, CommotError> {
        let file_metadata = metadata(&self.path)?;
        let file_size = FileSize::new(FileSizeType::new(file_metadata.len()));
        Ok(file_size)
        
    }


    pub fn created(&self) -> Result<DateTime<Utc>, CommotError> {
        let file_metadata = metadata(&self.path)?;
        Ok(file_metadata.created()?.into()) 
    }

    pub fn accessed(&self) -> Result<DateTime<Utc>, CommotError> {
        let file_metadata = metadata(&self.path)?;
        Ok(file_metadata.accessed()?.into()) 
    }
    pub fn modified(&self) -> Result<DateTime<Utc>, CommotError> {
        let file_metadata = metadata(&self.path)?;
        Ok(file_metadata.modified()?.into()) 
    }

    pub fn metadata(&self) -> Result<FileMetadata, CommotError> {
        Ok(
            FileMetadata::new(self.size()?, self.created()?, self.modified()?, self.accessed()?)
        )
    }
}