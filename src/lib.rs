// Modified from https://github.com/PistonDevelopers/find_folder/blob/master/src/lib.rs

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The search struct
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Search {
	path: String,
	depth: u8,
}

/// If the search was unsuccessful.
#[derive(Debug)]
pub enum Error {
    /// Some std io Error occurred.
    IO(::std::io::Error),
    /// The directory requested was not found.
    NotFound,
}

impl ::std::convert::From<io::Error> for Error {
    fn from(io_err: io::Error) -> Error {
        Error::IO(io_err)
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        writeln!(f, "{:?}", *self)
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO(ref io_err) => ::std::error::Error::description(io_err),
            Error::NotFound => "The folder could not be found",
        }
    }
}

impl Search {
	/// Search in assigned directory
	pub fn dir(path: &'static str) -> Search {
		Search { path: path.to_string(), depth: 1 }
	}

	///Search in current directory
	pub fn cwd() -> Search {
		let cwd = ::std::env::current_dir().unwrap();
		let string = cwd.as_path().to_str().unwrap();

		Search { path: string.to_string(), depth: 1 }	
	}

	/// Assign search depth, which defaults to 1
	pub fn with_depth(&mut self, depth: u8) -> &mut Search {
		self.depth = depth;
		self
	}

	/// Search for file and directory
	pub fn for_both(&mut self, file_path: &str) -> Result<PathBuf, Error> {
		let path = Path::new(&self.path);
		let dir_names: Vec<&str> = file_path.splitn(100, '/').collect();
		let head = dir_names[0];

		match check_kids(self.depth, head, path) {
			Ok(found_head) => {
				println!("Found head {:?}", found_head);
				let mut current_path = found_head;
				for i in 1..dir_names.len() {
					let result = check_dir(dir_names[i], current_path.as_path());
					if result.is_err() {
						return result;
					} else {
						current_path = result.ok().unwrap();
					}
				}
				Ok(current_path)
			},
			other_result => other_result,
		}
	}

	/// Search only for file
	pub fn for_file(&mut self, file_path: &str) -> Result<PathBuf, Error> {
		let result = self.for_both(file_path);
		if result.is_ok() && try!(fs::metadata(result.as_ref().unwrap().as_path())).is_dir() {
			return Err(Error::NotFound);
		} else {
			return result;
		}
	}

	/// Search only for directory
	pub fn for_folder(&mut self, file_path: &str) -> Result<PathBuf, Error> {
		let result = self.for_both(file_path);
		if result.is_ok() && !(try!(fs::metadata(result.as_ref().unwrap().as_path())).is_dir()) {
			return Err(Error::NotFound);
		} else {
			return result;
		}
	}
}

/// Check the contents of this folder and children folders.
pub fn check_kids(depth: u8, name: &str, path: &Path) -> Result<PathBuf, Error> {
    match check_dir(name, path) {
        err @ Err(Error::NotFound) => match depth > 0 {
            true => {
                for entry in try!(fs::read_dir(path)) {
                    let entry = try!(entry);
                    let entry_path = entry.path();
                    if try!(fs::metadata(&entry_path)).is_dir() {
                        if let Ok(folder) = check_kids(depth-1, name, &entry_path) {
                            return Ok(folder);
                        }
                    }
                }
                err
            },
            false => err,
        },
        other_result => other_result,
    }
}

/// Check the given directory for a folder with the matching name.
fn check_dir(name: &str, path: &Path) -> Result<PathBuf, Error> {
    for entry in try!(fs::read_dir(path)) {
        let entry = try!(entry);
        let entry_path = entry.path();
        if entry_path.ends_with(name) {
            return Ok(entry_path)
        }
    }
    Err(Error::NotFound)
}