use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

use fuse::mount;
use fuse::FileAttr;
use fuse::FileType;
use fuse::Filesystem;
use fuse::ReplyAttr;
use fuse::ReplyData;
use fuse::ReplyDirectory;
use fuse::ReplyEmpty;
use fuse::ReplyEntry;
use fuse::ReplyOpen;
use fuse::Request;

use libc::c_int;
use libc::ENODATA;
use libc::ENOENT;
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
impl Filesystem for WatchedFilesystem {
    /// Initialize the filesystem.  Called after mount.
    fn init(&mut self, _request: &Request) -> Result<(), c_int> {
        println!("Init");
        Ok(())
    }

    /// Presumably called to do tear down, but it's not at the moment.
    fn destroy(&mut self, _request: &Request) {
        // This doesn't appear to be called on umount...
        println!("Destroy")
    }

    /// Directory entry lookup.
    fn lookup(&mut self, request: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let name = name.to_str().unwrap();
        println!("Lookup: parent: {}, name: {}", parent, name);
        match name {
            "neato" => {
                let ttl = Timespec::new(1, 0);
                let file_attr = FileAttr {
                    ino: 2,
                    size: 8,
                    blocks: 1,
                    atime: time::get_time(),
                    mtime: time::get_time(),
                    ctime: time::get_time(),
                    crtime: time::get_time(),
                    kind: FileType::RegularFile,
                    perm: 0b100_000_000,
                    nlink: 1,
                    uid: request.uid(),
                    gid: request.gid(),
                    rdev: 0,
                    flags: 0,
                };
                reply.entry(&ttl, &file_attr, 0)
            }
            _ => {
                reply.error(ENOENT);
            }
        }
    }

    /// Appears to be part of the resource management.  Every time a lookup or
    /// create succeeds (and returns a inode), its counter is incremented by one.
    /// This is the decrement side, indicating that the resource is used by one
    /// less thing.  This is important to know when resources can be freed.
    fn forget(&mut self, _request: &Request, inode: u64, num_lookups: u64) {
        println!("Forget: inode: {}, num_lookups: {}", inode, num_lookups);
    }

    fn getattr(&mut self, request: &Request, inode: u64, reply: ReplyAttr) {
        println!("Getattr: inode: {}", inode);
        // Inode 1 appears to be the 'root' of the filesystem.
        if inode == 1 {
            let ttl = Timespec::new(1, 0);
            let file_attr = FileAttr {
                ino: inode,
                size: 1024,
                blocks: 1,
                atime: time::get_time(),
                mtime: time::get_time(),
                ctime: time::get_time(),
                crtime: time::get_time(),
                kind: FileType::Directory,
                perm: 0b101_000_000,
                nlink: 1,
                uid: request.uid(),
                gid: request.gid(),
                rdev: 0,
                flags: 0,
            };
            reply.attr(&ttl, &file_attr)
        } else if inode == 2 {
            let ttl = Timespec::new(1, 0);
            let file_attr = FileAttr {
                ino: inode,
                size: 8,
                blocks: 1,
                atime: time::get_time(),
                mtime: time::get_time(),
                ctime: time::get_time(),
                crtime: time::get_time(),
                kind: FileType::RegularFile,
                perm: 0b100_000_000,
                nlink: 1,
                uid: request.uid(),
                gid: request.gid(),
                rdev: 0,
                flags: 0,
            };
            reply.attr(&ttl, &file_attr)
        } else {
            reply.error(ENODATA);
        }
    }

    fn setattr(
        &mut self,
        _request: &Request,
        inode: u64,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<Timespec>,
        mtime: Option<Timespec>,
        fh: Option<u64>,
        crtime: Option<Timespec>,
        chgtime: Option<Timespec>,
        bkuptime: Option<Timespec>,
        flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        println!(
            "Setattr: inode: {}, mode: {:?}, uid: {:?}, gid: {:?} \
            size: {:?}, atime: {:?}, mtime: {:?}, fh: {:?}, crtime: {:?} \
            chgtime: {:?}, bkuptime: {:?}, flags: {:?}",
            inode, mode, uid, gid, size, atime, mtime, fh, crtime, chgtime, bkuptime, flags
        );
        reply.error(EPERM);
    }

    fn opendir(&mut self, _request: &Request, inode: u64, flags: u32, reply: ReplyOpen) {
        println!("Opendir: inode: {}, flags: {}", inode, flags);
        if inode == 1 {
            let file_handle: u64 = 0; // Don't need a file handle at this point in faking it.
            reply.opened(file_handle, flags);
        } else {
            reply.error(EPERM);
        }
    }

    fn readdir(
        &mut self,
        _request: &Request,
        inode: u64,
        handle: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        println!(
            "Readdir: inode: {}, handle: {}, offset: {}",
            inode, handle, offset
        );
        match (inode, offset) {
            (1, 0) => {
                reply.add(2, 1, FileType::RegularFile, "neato"); // Assuming at least one entry can be added.
                reply.ok();
            }
            (1, 1) => {
                reply.ok();
            }
            (_, _) => {
                reply.error(EPERM);
            }
        }
    }

    fn releasedir(
        &mut self,
        _request: &Request,
        inode: u64,
        handle: u64,
        flags: u32,
        reply: ReplyEmpty,
    ) {
        println!(
            "Releasedir: inode: {}, handle: {}, flags: {}",
            inode, handle, flags
        );
        if inode == 1 {
            reply.ok();
        } else {
            reply.error(EPERM);
        }
    }

    fn open(&mut self, _request: &Request, inode: u64, flags: u32, reply: ReplyOpen) {
        println!("Open: inode: {}, flags: {}", inode, flags);
        if inode == 2 {
            let file_handle: u64 = 42;
            reply.opened(file_handle, flags);
        } else {
            reply.error(EPERM);
        }
    }

    fn read(
        &mut self,
        _request: &Request,
        inode: u64,
        file_handle: u64,
        offset: i64,
        size: u32,
        reply: ReplyData,
    ) {
        println!(
            "Read: inode: {}, file_handle: {}, offset: {}, size: {}",
            inode, file_handle, offset, size
        );
        let data = "burrito\n".as_bytes();
        let offset = offset as usize;
        let size = size as usize;
        match (inode, file_handle) {
            (2, 42) => {
                if offset < 8 {
                    let end = usize::min(8, offset + size);
                    reply.data(&data[offset..end]);
                } else {
                    reply.error(ENXIO);
                }
            }
            _ => {
                reply.error(ENXIO);
            }
        }
    }

    fn release(
        &mut self,
        _request: &Request,
        inode: u64,
        file_handle: u64,
        flags: u32,
        lock_owner: u64,
        flush: bool,
        reply: ReplyEmpty,
    ) {
        println!(
            "Release: inode: {}, file_handle: {}, flags: {}, lock_owner: {}, flush: {}",
            inode, file_handle, flags, lock_owner, flush
        );
        reply.ok();
    }
}

pub fn watch_filesystem(mount_point: &Path, backing_directory: &Path) {
    let watched_fs = WatchedFilesystem::new(backing_directory);
    let options: &[&OsStr] = &[];
    let result = mount(watched_fs, &mount_point, options);
    match result {
        Ok(_) => {
            println!("Unmounted");
        }
        Err(error) => {
            eprintln!("Failed: {}", error);
        }
    }
}
