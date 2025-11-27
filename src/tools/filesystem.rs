// filesystem.rs

use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use crate::error::Result; 

pub struct FileSystem;

impl FileSystem {
    pub fn new() -> Self {
        FileSystem
    }
    
    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        Ok(content)
    }
    
    pub fn read_file_lines<P: AsRef<Path>>(&self, path: P, startline: u32, endline: u32) -> Result<Vec<String>> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        let lines = content.lines().collect::<Vec<_>>();
        let start = (startline - 1) as usize;
        let end = (endline - 1) as usize;
        Ok(lines[start..=end].iter().map(|s| s.to_string()).collect())
    }

    pub fn list_directory<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let path = path.as_ref();
        let entries = fs::read_dir(path)?;
        let mut result = Vec::new();
        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let file_type = if metadata.is_dir() {
                FileType::Directory
            } else if metadata.is_file() {
                FileType::File
            } else {
                FileType::Other
            };
            result.push(FileEntry {
                name: entry.file_name().into_string().unwrap(),
                file_type,
                size: metadata.len(),
                path: entry.path(),
            });
        }
        Ok(serde_json::to_string_pretty(&result)?)
    }
}

/// 파일 항목 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub file_type: FileType,
    pub size: u64,
    pub path: PathBuf,
}

/// 파일 타입
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    File,
    Directory,
    Other,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_readfile() {
        let fs = FileSystem::new();
        let content = fs.read_file("./Cargo.toml").unwrap();
        println!("{}", content);
    }

    #[test]
    fn test_notfound() {
        let fs = FileSystem::new();
        let content = fs.read_file("./notfound");
        assert!(content.is_err(), "What the?");
        println!("{}", content.unwrap_err());
    }
    
    #[test]
    fn test_list_directory_json() {
        let fs = FileSystem::new();
        let json = fs.list_directory("./src").unwrap();
        println!("JSON:\n{}", json);
        // JSON이 제대로 파싱되는지 확인
        let parsed: Vec<FileEntry> = serde_json::from_str(&json).unwrap();
        assert!(!parsed.is_empty());
    }
}
