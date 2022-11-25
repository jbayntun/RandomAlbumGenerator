use log::debug;
use math::round;
use rand::seq::SliceRandom;
use std::ffi::OsStr;
use std::fs::{self, DirEntry};
use std::path::Path;

use anyhow::Result;
use nu_glob::glob;
use thiserror::Error;

#[derive(Debug)]
pub struct Album {
    pub name: String,
    photos: Vec<String>,
    top_photos: Vec<String>,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AlbumError {
    #[error("Not a valid root directory.")]
    InvalidRoot,
}

static IMAGE_TYPES: [&str; 1] = [".png"];

/// Returns a specified amount of random photos from an album.
pub fn get_randoms_from_album(album: &Album, count: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let top_target = round::half_up(0.75 * count as f64, 0) as usize;

    for pic in album.top_photos[..].choose_multiple(&mut rand::thread_rng(), top_target) {
        result.push(pic.to_string());
    }

    for pic in album.photos[..].choose_multiple(&mut rand::thread_rng(), count - result.len()) {
        result.push(pic.to_string());
    }

    result
}

/// Recursively checks directories for photos and creates albums.
/// If an error occurs in any given directory, it will be silently ignored
/// in the hopes that other albums will succeed.
pub fn get_albums(root_dir: &str) -> Result<Vec<Album>, AlbumError> {
    let path = Path::new(root_dir);
    if !path.is_dir() {
        return Err(AlbumError::InvalidRoot);
    }

    let mut albums = Vec::new();
    let mut paths_to_check = vec![root_dir.to_string()];

    loop {
        if let Some(curr_path) = paths_to_check.pop() {
            let mut photos: Vec<String> = Vec::new();
            let mut top_photos: Vec<String> = Vec::new();

            add_photos(&curr_path, &mut photos);

            let entries = match Path::new(&curr_path).read_dir() {
                Ok(x) => x,
                Err(e) => {
                    debug!("Error for read dir in path: {}\n{}", curr_path, e);
                    continue;
                }
            };

            // look for child directories and either get "top" pics or add the
            // directories to the list to check.
            for entry in entries {
                if let Some((dir_name, dir_path)) = get_dir(entry) {
                    if dir_name.eq_ignore_ascii_case("Top") {
                        add_photos(&dir_path, &mut top_photos);
                    } else {
                        paths_to_check.push(dir_path);
                    }
                }
            }

            // if we have any photos, push an album.
            if photos.len() > 0 || top_photos.len() > 0 {
                if let Some(name) = osstr_to_str(Path::new(&curr_path).file_name()) {
                    albums.push(Album {
                        name: name.to_string(),
                        photos: photos,
                        top_photos: top_photos,
                    });
                };
            }
        } else {
            // there are no directories left to search
            break;
        }
    }

    Ok(albums)
}

/// adds all photos (the path of) in the directory to the photos vector.
fn add_photos(path: &str, photos: &mut Vec<String>) {
    for e in IMAGE_TYPES {
        let pat = path.to_owned() + "/*" + e;
        for entry in glob(&pat).expect("Failed to read glob pattern") {
            if let Ok(path_buf) = entry {
                if let Ok(cannonical) = fs::canonicalize(path_buf) {
                    if let Some(pic_str) = cannonical.to_str() {
                        photos.push(pic_str.to_string());
                    }
                }
            }
        }
    }
}

/// Returns a tuple (DirName, DirPath)
/// DirPath includes the name
fn get_dir(de: Result<DirEntry, std::io::Error>) -> Option<(String, String)> {
    if let Ok(de) = de {
        let path_buf = de.path();

        if path_buf.is_dir() {
            if let Some(path_str) = path_buf.as_os_str().to_str() {
                if let Some(name_str) = osstr_to_str(path_buf.file_name()) {
                    return Some((name_str.to_string(), path_str.to_string()));
                }
            }
        }
    }

    debug!("In get_dir and we were unable to convert to string");

    None
}

/// Utility to get a &str from an OsStr.
fn osstr_to_str(os: Option<&OsStr>) -> Option<&str> {
    if let Some(os_str) = os {
        if let Some(str) = os_str.to_str() {
            return Some(str);
        }
    }

    debug!("In osstr_to_str and we were unable to convert to string");

    None
}

#[cfg(test)]
mod test {
    use anyhow::Ok;

    use super::*;

    #[test]
    #[should_panic]
    fn invalid_root() {
        get_albums("fsdklfj/fjlasd").unwrap();
    }

    #[test]
    fn empty_dir() {
        assert_eq!(0, get_albums("test_items/empty_dir").unwrap().len());
    }

    #[test]
    fn no_pics() {
        let albums = get_albums("test_items/no_pics").unwrap();
        assert_eq!(0, albums.len());
    }

    #[test]
    fn has_pics() {
        let mut albums = get_albums("test_items/pics").unwrap();
        assert_eq!(1, albums.len());

        let a = albums.pop().unwrap();
        assert_eq!(8, a.photos.len());
        assert_eq!(0, a.top_photos.len());
    }

    #[test]
    fn pics_top() {
        let mut albums = get_albums("test_items/Root Album").unwrap();
        assert_eq!(1, albums.len());

        let a = albums.pop().unwrap();
        assert_eq!(5, a.photos.len());
        assert_eq!(3, a.top_photos.len());
    }

    #[test]
    fn multi_album() {
        let albums = get_albums("test_items/multi_album").unwrap();
        assert_eq!(3, albums.len());
    }

    #[test]
    fn adding_photos() {
        let mut vec: Vec<String> = Vec::new();
        add_photos("test_items/Root Album", &mut vec);
        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn some_random() -> Result<()> {
        let mut albums = get_albums("test_items/Root Album")?;
        let a = albums
            .pop()
            .ok_or_else(|| anyhow::anyhow!("string conversion failed in get_dir()"))?;

        assert_eq!(4, get_randoms_from_album(&a, 4).len());

        Ok(())
    }
}
