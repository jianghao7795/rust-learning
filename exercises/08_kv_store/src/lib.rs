use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

const SET: u8 = 1;
const REMOVE: u8 = 2;

#[derive(Debug)]
pub enum StoreError {
    Io(io::Error),
    Corrupt(String),
    EmptyKey,
    ValueTooLarge,
}

impl fmt::Display for StoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "存储 I/O 错误：{error}"),
            Self::Corrupt(message) => write!(formatter, "存储文件损坏：{message}"),
            Self::EmptyKey => formatter.write_str("键不能为空"),
            Self::ValueTooLarge => formatter.write_str("键或值过大"),
        }
    }
}

impl Error for StoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for StoreError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    values: HashMap<String, String>,
}

impl KvStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let path = path.as_ref().to_path_buf();
        let mut values = HashMap::new();

        match File::open(&path) {
            Ok(mut file) => replay(&mut file, &mut values)?,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                File::create(&path)?;
            }
            Err(error) => return Err(error.into()),
        }

        Ok(Self { path, values })
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<Option<String>, StoreError> {
        validate_key(key)?;
        append_record(&self.path, SET, key, value)?;
        Ok(self.values.insert(key.to_string(), value.to_string()))
    }

    pub fn remove(&mut self, key: &str) -> Result<Option<String>, StoreError> {
        validate_key(key)?;
        if !self.values.contains_key(key) {
            return Ok(None);
        }
        append_record(&self.path, REMOVE, key, "")?;
        Ok(self.values.remove(key))
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

fn validate_key(key: &str) -> Result<(), StoreError> {
    if key.is_empty() {
        Err(StoreError::EmptyKey)
    } else {
        Ok(())
    }
}

fn append_record(path: &Path, operation: u8, key: &str, value: &str) -> Result<(), StoreError> {
    let key_length = u32::try_from(key.len()).map_err(|_| StoreError::ValueTooLarge)?;
    let value_length = u32::try_from(value.len()).map_err(|_| StoreError::ValueTooLarge)?;
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(&[operation])?;
    file.write_all(&key_length.to_le_bytes())?;
    file.write_all(&value_length.to_le_bytes())?;
    file.write_all(key.as_bytes())?;
    file.write_all(value.as_bytes())?;
    file.flush()?;
    Ok(())
}

fn replay(file: &mut File, values: &mut HashMap<String, String>) -> Result<(), StoreError> {
    loop {
        let mut operation = [0_u8; 1];
        match file.read_exact(&mut operation) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => return Ok(()),
            Err(error) => return Err(error.into()),
        }
        let key_length = read_length(file)?;
        let value_length = read_length(file)?;
        let key = read_string(file, key_length)?;
        let value = read_string(file, value_length)?;

        match operation[0] {
            SET => {
                values.insert(key, value);
            }
            REMOVE => {
                values.remove(&key);
            }
            other => return Err(StoreError::Corrupt(format!("未知操作码 {other}"))),
        }
    }
}

fn read_length(file: &mut File) -> Result<usize, StoreError> {
    let mut bytes = [0_u8; 4];
    file.read_exact(&mut bytes)
        .map_err(|error| StoreError::Corrupt(format!("记录长度不完整：{error}")))?;
    Ok(u32::from_le_bytes(bytes) as usize)
}

fn read_string(file: &mut File, length: usize) -> Result<String, StoreError> {
    let mut bytes = vec![0; length];
    file.read_exact(&mut bytes)
        .map_err(|error| StoreError::Corrupt(format!("记录内容不完整：{error}")))?;
    String::from_utf8(bytes).map_err(|error| StoreError::Corrupt(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_path(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("stage08-{name}-{}-{unique}.kv", std::process::id()))
    }

    #[test]
    fn persists_updates_and_removals() {
        let path = test_path("persistence");
        {
            let mut store = KvStore::open(&path).expect("store should open");
            store.set("language", "Rust").expect("set should work");
            store.set("edition", "2024").expect("set should work");
            store.remove("edition").expect("remove should work");
        }

        let store = KvStore::open(&path).expect("store should reopen");
        assert_eq!(store.get("language"), Some("Rust"));
        assert_eq!(store.get("edition"), None);
        assert_eq!(store.len(), 1);
        drop(store);
        std::fs::remove_file(path).expect("test file should be removable");
    }

    #[test]
    fn rejects_an_empty_key() {
        let path = test_path("empty-key");
        let mut store = KvStore::open(&path).expect("store should open");
        assert!(matches!(store.set("", "value"), Err(StoreError::EmptyKey)));
        drop(store);
        std::fs::remove_file(path).expect("test file should be removable");
    }
}
