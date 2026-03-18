use serde::{Serialize, de::DeserializeOwned};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

pub fn save_json<T: Serialize>(
    path: &str,
    value: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let path_obj = Path::new(path);

    if let Some(parent) = path_obj.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let file = File::create(path_obj)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, value)?;
    Ok(())
}

pub fn load_json<T: DeserializeOwned>(
    path: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let value = serde_json::from_reader(reader)?;
    Ok(value)
}

pub fn load_or_compute<T, F>(
    path: &str,
    compute: F,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: Serialize + DeserializeOwned,
    F: FnOnce() -> T,
{
    if Path::new(path).exists() {
        println!("🔁 loading cache: {}", path);
        return load_json(path);
    }

    println!("⚙️ computing: {}", path);
    let value = compute();
    save_json(path, &value)?;
    Ok(value)
}

#[derive(Debug, Clone)]
pub struct Cache {
    base_dir: PathBuf,
}

pub fn cache() -> Cache {
    Cache {
        base_dir: PathBuf::from("out/cache"),
    }
}

impl Cache {
    pub fn under(mut self, subdir: &str) -> Self {
        self.base_dir.push(subdir);
        self
    }

    pub fn path(&self, key: &str) -> String {
        self.base_dir
            .join(format!("{}.json", key))
            .to_string_lossy()
            .into_owned()
    }

    pub fn load_or_compute<T, F>(
        &self,
        key: &str,
        compute: F,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        T: Serialize + DeserializeOwned,
        F: FnOnce() -> T,
    {
        let path = self.path(key);
        load_or_compute(&path, compute)
    }
}