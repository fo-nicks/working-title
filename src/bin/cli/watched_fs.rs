use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use fuse_mt::mount;
use fuse_mt::CallbackResult;
use fuse_mt::DirectoryEntry;
use fuse_mt::FileAttr;
use fuse_mt::FileType;
use fuse_mt::FilesystemMT;
use fuse_mt::FuseMT;
use fuse_mt::RequestInfo;
use fuse_mt::ResultEmpty;
use fuse_mt::ResultEntry;
use fuse_mt::ResultOpen;
use fuse_mt::ResultReaddir;
use fuse_mt::ResultSlice;
use fuse_mt::ResultWrite;

use libc::ENODATA;
use libc::ENXIO;
use libc::EPERM;

use time::Timespec;

/// Watched filesystem watches a filesystem for activity and outputs messages to
/// stdout as activity occurs.  This implementation uses FUSE and a backing
/// directory.

struct WatchedFilesystem {
    _backing_directory: PathBuf,
}

impl WatchedFilesystem {
    fn new(backing_directory: &Path) -> WatchedFilesystem {
        WatchedFilesystem {
            _backing_directory: backing_directory.to_path_buf(),
        }
    }
}

/// At this point, the WatchedFilesystem presents a single file with fixed context.
/// It will also spew about a bunch of messages about various methods being called.
/// (Obviously, only a proof that FUSE can do _something_.
impl FilesystemMT for WatchedFilesystem {
    /// Initialize the filesystem.  Called after mount.
    fn init(&self, _request: RequestInfo) -> ResultEmpty {
        println!("Init");
        Ok(())
    }

    /// Presumably called to do tear down, but it's not at the moment.
    fn destroy(&self, _request: RequestInfo) {
        println!("Destroy");
    }

    fn getattr(
        &self,
        request: RequestInfo,
        path: &Path,
        file_handle: Option<u64>,
    ) -> ResultEntry {
        println!("Getattr: path: {}, file_handle: {:?}", path.display(), file_handle);
        if path == Path::new("/") {
            let ttl = Timespec::new(1, 0);
            let file_attr = FileAttr {
                size: 1024,
                blocks: 1,
                atime: time::get_time(),
                mtime: time::get_time(),
                ctime: time::get_time(),
                crtime: time::get_time(),
                kind: FileType::Directory,
                perm: 0b101_000_000,
                nlink: 1,
                uid: request.uid,
                gid: request.gid,
                rdev: 0,
                flags: 0,
            };
            Ok((ttl, file_attr))
        } else if path == Path::new("/neato") {
            let ttl = Timespec::new(1, 0);
            let file_attr = FileAttr {
                size: 8,
                blocks: 1,
                atime: time::get_time(),
                mtime: time::get_time(),
                ctime: time::get_time(),
                crtime: time::get_time(),
                kind: FileType::RegularFile,
                perm: 0b100_000_000,
                nlink: 1,
                uid: request.uid,
                gid: request.gid,
                rdev: 0,
                flags: 0,
            };
            Ok((ttl, file_attr))
        } else {
            Err(ENODATA)
        }
    }

    fn opendir(&self, _request: RequestInfo, path: &Path, flags: u32) -> ResultOpen {
        println!("Opendir: path: {}, flags: {}", path.display(), flags);
        if path == Path::new("/") {
            let file_handle: u64 = 0; // Don't need a file handle at this point in faking it.
            Ok((file_handle, flags))
        } else {
            Err(EPERM)
        }
    }

    fn readdir(&self, _request: RequestInfo, path: &Path, handle: u64) -> ResultReaddir {
        println!("Readdir: path: {}, handle: {}", path.display(), handle);
        if path == Path::new("/") {
            let mut entries = Vec::new();
            entries.push(DirectoryEntry {
                name: OsString::from_str("neato").unwrap(),
                kind: FileType::RegularFile,
            });
            Ok(entries)
        } else {
            Err(EPERM)
        }
    }

    fn releasedir(&self, _request: RequestInfo, path: &Path, handle: u64, flags: u32) -> ResultEmpty {
        println!("Releasedir: path: {}, handle: {}, flags: {}", path.display(), handle, flags);
        if path == Path::new("/") {
            Ok(())
        } else {
            Err(EPERM)
        }
    }

    fn open(&self, _request: RequestInfo, path: &Path, flags: u32) -> ResultOpen {
        println!("Open: path: {}, flags: {}", path.display(), flags);
        if path == Path::new("/neato") {
            let file_handle: u64 = 42;
            Ok((file_handle, flags))
        } else {
            Err(EPERM)
        }
    }

    fn read(
        &self,
        _request: RequestInfo,
        path: &Path,
        file_handle: u64,
        offset: u64,
        size: u32,
        callback: impl FnOnce(ResultSlice<'_>) -> CallbackResult
    ) -> CallbackResult {
        println!(
            "Read: path: {}, file_handle: {}, offset: {}, size: {}",
            path.display(), file_handle, offset, size
        );
        let data = "burrito\n".as_bytes();
        let offset = offset as usize;
        let size = size as usize;
        let _neato_path = Path::new("/neato");
        let result: ResultSlice<'_> = match (path, file_handle) {
            (_neato_path, 42) => {
                if offset < 8 {
                    let end = usize::min(8, offset + size);
                    Ok(&data[offset..end])
                } else {
                    Err(ENXIO)
                }
            }
            _ => Err(ENXIO),
        };
        callback(result)
    }

    fn write(
        &self,
        _request: RequestInfo,
        path: &Path,
        file_handle: u64,
        offset: u64,
        data: Vec<u8>,
        flags: u32,
    ) -> ResultWrite {
        println!(
            "Write: path: {}, file_handle: {}, offset: {}, data (len): {}, flags: {}",
            path.display(),
            file_handle,
            offset,
            data.len(),
            flags
        );
        Err(EPERM)
    }

    fn release(
        &self,
        _request: RequestInfo,
        path: &Path,
        file_handle: u64,
        flags: u32,
        lock_owner: u64,
        flush: bool,
    ) -> ResultEmpty {
        println!(
            "Release: path: {}, file_handle: {}, flags: {}, lock_owner: {}, flush: {}",
            path.display(), file_handle, flags, lock_owner, flush
        );
        Ok(())
    }
}

pub fn watch_filesystem(mount_point: &Path, backing_directory: &Path) {
    let watched_fs = WatchedFilesystem::new(backing_directory);
    let options: &[&OsStr] = &[];
    let fuse_mt = FuseMT::new(watched_fs, 2);
    let result = mount(fuse_mt, &mount_point, options);
    match result {
        Ok(_) => {
            println!("Unmounted");
        }
        Err(error) => {
            eprintln!("Failed: {}", error);
        }
    }
}
