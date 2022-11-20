use log::debug;
use std::ffi::OsStr;
use std::fs::DirEntry;
use std::path::Path;

use anyhow::Result;
use nu_glob::glob;
use thiserror::Error;

#[derive(Debug)]
pub struct Album {
    name: String,
    photos: Vec<String>,
    top_photos: Vec<String>,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AlbumError {
    #[error("Not a valid root directory.")]
    InvalidRoot,
    #[error("Cannot use empty root directory.")]
    EmptyRoot,
    #[error("Expected directory.")]
    ExpectedDir,
}

static IMAGE_TYPES: [&str; 3] = [".png", ".jpg", ".jpeg"];

fn add_photos(path: &str, photos: &mut Vec<String>) {
    for e in IMAGE_TYPES {
        let pat = path.to_owned() + "/*" + e;
        for entry in glob(&pat).expect("Failed to read glob pattern") {
            if let Ok(entry) = entry {
                if let Some(pic) = entry.file_name().and_then(OsStr::to_str) {
                    photos.push(pic.to_string());
                }
            }
        }
    }
}

fn get_dir(de: Result<DirEntry, std::io::Error>) -> Result<String> {
    let path = de?.path();
    if !path.is_dir() {
        return Err(anyhow::anyhow!("not a dir, looking for dirs here"));
    }

    let d = path
        .as_os_str()
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("string conversion failed in get_dir()"))?;
    Ok(d.to_string())
}

fn get_albums(root_dir: &str) -> Result<Vec<Album>, AlbumError> {
    let path = Path::new(root_dir);
    if !path.is_dir() {
        return Err(AlbumError::InvalidRoot);
    }

    let mut albums = Vec::new();
    let mut paths_to_check = vec![path];
    loop {
        if let Some(curr_path) = paths_to_check.pop() {
            let mut photos: Vec<String> = Vec::new();
            let mut top_photos: Vec<String> = Vec::new();

            if let Some(path_str) = curr_path.to_str() {
                add_photos(path_str, &mut photos);

                for entry in curr_path.read_dir().expect("read_dir call failed") {
                    match get_dir(entry) {
                        Ok(d) => {
                            if d.eq_ignore_ascii_case("top") {
                                add_photos(&d, &mut top_photos);
                            } else {
                                paths_to_check.push(path);
                            }
                        }
                        Err(e) => {
                            debug!("Error getting dir: {}", e);
                            continue;
                        }
                    }
                }

                if photos.len() > 0 || top_photos.len() > 0 {
                    albums.push(Album {
                        name: curr_path
                            .file_name()
                            .unwrap_or(OsStr::new("hellofunnybad"))
                            .to_str()
                            .unwrap_or("hellootherbad")
                            .to_string(),
                        photos: photos,
                        top_photos: top_photos,
                    });
                }
            } else {
                debug!("curr_path.to_str() failed");
                continue;
            }
        } else {
            break;
        }

        // create album
        // let mut a = Some(Album::new(
        //     curr_path.file_name().unwrap().to_str().unwrap().to_string(),
        // ));
    }

    if albums.is_empty() {
        return Err(AlbumError::EmptyRoot);
    }

    Ok(albums)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_root() {
        get_albums("fsdklfj/fjlasd").unwrap();
    }

    #[test]
    fn empty_dir() {
        assert_eq!(
            0,
            get_albums("/Users/jeffb/Projects/rust_basic/image_play/test_items/empty_dir")
                .unwrap()
                .len()
        );
    }

    #[test]
    fn no_pics() {
        assert_eq!(
            0,
            get_albums("/Users/jeffb/Projects/rust_basic/image_play/test_items/no_pics")
                .unwrap()
                .len()
        );
    }

    #[test]
    fn pics() {
        assert_eq!(
            5,
            get_albums("/Users/jeffb/Projects/rust_basic/image_play/test_items/no_pics")
                .unwrap()
                .len()
        );
    }

    #[test]
    fn pics_top() {
        assert_eq!(
            1,
            get_albums("/Users/jeffb/Projects/rust_basic/image_play/test_items/Root Album")
                .unwrap()
                .len()
        );
    }

    #[test]
    fn adding_photos() {
        let mut vec: Vec<String> = Vec::new();
        add_photos(
            "/Users/jeffb/Projects/rust_basic/image_play/test_items/Root Album",
            &mut vec,
        );
        assert_eq!(vec.len(), 5);
    }
}
