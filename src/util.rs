use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::fs::metadata;

pub fn copy_recursive_filter<F>(source: &Path, dest: &Path, valid: &F) -> io::Result<()>
    where F: Fn(&Path) -> bool
{
    if metadata(&source).unwrap().is_dir() {
        for entry in try!(fs::read_dir(source)) {
            let entry = try!(entry).path();
            if metadata(&entry).unwrap().is_dir() {
                if valid(entry.as_path()) {
                    let real_entry = entry.to_str().unwrap().split("/").last().unwrap();
                    let new_dest = &dest.join(real_entry);
                    try!(fs::create_dir_all(new_dest));
                    try!(copy_recursive_filter(entry.as_path(), new_dest, valid));
                }
            } else {
                if valid(entry.as_path()) {
                    let real_entry = entry.to_str().unwrap().split("/").last().unwrap();
                    try!(fs::copy(entry.as_path(), &dest.join(real_entry)));
                }
            }
        }
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other,
                           "source parameter needs to be a directory"))
    }
}

// one possible implementation of fs::walk_dir only visiting files
pub fn walk_dir(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(walk_dir(&entry.path(), cb));
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
