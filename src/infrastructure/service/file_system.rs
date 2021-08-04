use anyhow::Result;
use color_eyre::{eyre::eyre, Report};
use hex;
use sha2::{Digest, Sha256};
use std::{env, fs, fs::File, io::prelude::*, path};
use tracing::error;

pub static BASE64_ENCODED_SOURCE_EXTENSION: &str = ".rs.b64";

fn hash_content(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let hash = hasher.finalize().to_vec();

    hex::encode(hash).chars().into_iter().take(10).collect()
}

/// Ensure a directory exists in the file system
///
/// # Examples
///
/// ```
/// use safepkt_server::infra::file_system;
/// use std::path::Path;
/// use std::fs;
///
/// let file_path = "/tmp/deep/down/file-which-does-not-exist-yet";
///
/// assert!(!Path::exists(Path::new(file_path)));
///
/// match file_system::ensure_directory_exists(file_path) {
///     Ok(path) => assert_eq!(
///         file_path,
///         path.to_str().unwrap()
///     ),
///     Err(_) => unreachable!()
/// }
///
/// assert!(Path::exists(Path::new(file_path)));
///
/// fs::remove_dir_all("/tmp/deep");
/// ```
///
pub fn ensure_directory_exists(path_as_str: &str) -> Result<&path::Path, Report> {
    let path = path::Path::new(path_as_str);

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    Ok(path)
}

/// Guard against missing file in file system  
/// located at path provided as argument
///
/// # Examples
///
/// ```
/// use safepkt_server::infra::file_system;
/// use std::env;
/// use std::path::Path;
///
/// let file = "/tmp/non-existing-file";
///
/// match file_system::guard_against_missing_source(file) {
///     Err(report) => assert_eq!(
///         "Can not find file at path \"/tmp/non-existing-file\"".to_string(),
///         report.to_string()
///     ),
///     Ok(_) => unreachable!()
/// }
/// ```
///
pub fn guard_against_missing_source(path_as_str: &str) -> Result<(), Report> {
    let source_path = path::Path::new(path_as_str);

    if !source_path.exists() {
        let error_message = format!("Can not find file at path \"{}\"", path_as_str);
        error!("{}", error_message);

        return Err(eyre!(error_message));
    }

    Ok(())
}

/// Get the directory where source files are saved.  
/// Its path is declared in the root configuration file (.env).  
/// This path is declared as the value of the SOURCE_DIRECTORY environment variable.  
///
/// # Examples
///
/// ```
/// use safepkt_server::infra::file_system;
/// use std::env;
/// use std::path::Path;
///
/// let expected_uploaded_source_directory_path = "/tmp";
/// env::set_var("SOURCE_DIRECTORY", "/tmp");
///
/// let actual_path = file_system::get_uploaded_source_directory().unwrap();
/// assert_eq!(expected_uploaded_source_directory_path, actual_path);
/// ```
///
pub fn get_uploaded_source_directory() -> Result<String, Report> {
    let source_directory = env::var("SOURCE_DIRECTORY")?;
    Ok(source_directory)
}

/// Save bytes content to a file in the file system.  
/// The file is written in a directory,
/// which path is in the root configuration file (.env).  
/// This path is declared as the value of the SOURCE_DIRECTORY environment variable.  
/// The file name is a truncated hash of the content to save,  
/// concatenated with an extension.  
///
/// # Examples
///
/// ```
/// use safepkt_server::infra::file_system;
/// use std::env;
/// use std::path::Path;
///
/// env::set_var("SOURCE_DIRECTORY", "/tmp");
/// let file_path = file_system::save_content_in_file_system("my contents".as_bytes()).unwrap();
/// assert!(Path::exists(Path::new(file_path.as_str())));
/// ```
///
pub fn save_content_in_file_system(content: &[u8]) -> Result<String, Report> {
    let content_hash: String = hash_content(content);
    let uploaded_source_directory = get_uploaded_source_directory()?;
    let file_name = format!("{}{}", content_hash, BASE64_ENCODED_SOURCE_EXTENSION);
    let file_path =
        [uploaded_source_directory, file_name].join(path::MAIN_SEPARATOR.to_string().as_str());

    let mut file = File::create(file_path.clone())?;
    file.write_all(content)?;

    Ok(file_path)
}
