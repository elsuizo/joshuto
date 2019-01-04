extern crate libc;
extern crate ncurses;

use std::path;
use std::process;

pub const BITMASK  : u32 = 0o170000;
pub const S_IFSOCK : u32 = 0o140000;   /* socket */
pub const S_IFLNK  : u32 = 0o120000;   /* symbolic link */
pub const S_IFREG  : u32 = 0o100000;   /* regular file */
pub const S_IFBLK  : u32 = 0o060000;   /* block device */
pub const S_IFDIR  : u32 = 0o040000;   /* directory */
pub const S_IFCHR  : u32 = 0o020000;   /* character device */
pub const S_IFIFO  : u32 = 0o010000;   /* FIFO */

/*
pub const fn is_reg(mode: u32) -> bool
{
    mode >> 9 & S_IFREG >> 9 == mode >> 9
}

pub fn get_unix_filetype(mode : u32) -> &'static str
{
    match mode & BITMASK {
        S_IFBLK => "inode/blockdevice",
        S_IFCHR => "inode/chardevice",
        S_IFDIR => "inode/directory",
        S_IFIFO => "inode/fifo",
        S_IFLNK => "inode/symlink",
        S_IFSOCK => "inode/socket",
        S_IFREG => "inode/regular",
        _ => "unknown",
    }
}
*/

pub fn is_executable(mode : u32) -> bool
{
    const LIBC_PERMISSION_VALS : [ u32 ; 3] = [
            libc::S_IXUSR,
            libc::S_IXGRP,
            libc::S_IXOTH,
        ];

    for val in LIBC_PERMISSION_VALS.iter() {
        if mode & val != 0 {
            return true;
        }
    }
    return false;
}

pub fn stringify_mode(mode: u32) -> String
{
    let mut mode_str: String = String::with_capacity(10);

    const LIBC_FILE_VALS: [(u32, char) ; 7] = [
        (S_IFREG, '-'),
        (S_IFDIR, 'd'),
        (S_IFLNK, 'l'),
        (S_IFSOCK, 's'),
        (S_IFBLK, 'b'),
        (S_IFCHR, 'c'),
        (S_IFIFO, 'f'),
    ];

    const LIBC_PERMISSION_VALS : [(u32, char) ; 9] = [
            (libc::S_IRUSR, 'r'),
            (libc::S_IWUSR, 'w'),
            (libc::S_IXUSR, 'x'),
            (libc::S_IRGRP, 'r'),
            (libc::S_IWGRP, 'w'),
            (libc::S_IXGRP, 'x'),
            (libc::S_IROTH, 'r'),
            (libc::S_IWOTH, 'w'),
            (libc::S_IXOTH, 'x'),
    ];

    let mode_shifted = mode >> 9;
    for val in LIBC_FILE_VALS.iter() {
        let val_shifted = val.0 >> 9;
        if mode_shifted & val_shifted == mode_shifted {
            mode_str.push(val.1);
            break;
        }
    }

    for val in LIBC_PERMISSION_VALS.iter() {
        if mode & val.0 != 0 {
            mode_str.push(val.1);
        } else {
            mode_str.push('-');
        }
    }
    mode_str
}

pub fn open_with(path: &path::Path, args: &Vec<String>)
{
    let program = args[0].clone();
    let args_len = args.len();

    let mut command = process::Command::new(program);
    for i in 1..args_len {
        command.arg(args[i].clone());
    }
    command.arg(path.as_os_str());

    match command.spawn() {
        Ok(mut handle) => {
            match handle.wait() {
                Ok(_) => {},
                Err(e) => eprintln!("{}", e),
            }
        },
        Err(e) => {
            eprintln!("{:?}", e);
        },
    }
}
