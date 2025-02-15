use core::ops::{Deref, DerefMut};

use alloc::{string::String, vec::Vec};

pub fn open(str: &str, mode: usize) -> isize {
    syscall!(sc::nr::OPEN, str.as_ptr() as usize, mode, str.len())
}

pub fn close(fd: usize) {
    syscall!(sc::nr::CLOSE, fd);
}

pub fn read(fd: usize, buf: usize, len: usize) -> isize {
    syscall!(sc::nr::READ, fd, buf, len)
}

pub fn write(fd: usize, buf: usize, len: usize) -> isize {
    syscall!(sc::nr::WRITE, fd, buf, len)
}

pub fn fstat(fd: usize, buf: usize) -> isize {
    syscall!(sc::nr::FSTAT, fd, buf)
}

pub fn pipe(fd: usize) -> isize {
    syscall!(sc::nr::PIPE, fd)
}

pub fn lseek(fd: usize, offset: usize) -> isize {
    syscall!(sc::nr::LSEEK, fd, offset)
}

pub fn ioctl(fd: usize, cmd: usize, arg: usize) -> isize {
    syscall!(sc::nr::IOCTL, fd, cmd, arg)
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum InodeTy {
    Dir = 0,
    #[default]
    File = 1,
}

#[repr(C)]
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileInfo {
    pub ty: InodeTy,
    pub name: String,
}

impl FileInfo {
    pub fn new(name: String, ty: InodeTy) -> Self {
        Self { ty, name }
    }
}

pub fn dir_item_num(fd: usize) -> isize {
    syscall!(10006, fd)
}

pub fn list_dir(fd: usize) -> Vec<FileInfo> {
    let len = dir_item_num(fd) as usize;
    let mut buf = alloc::vec![FileInfo::default(); len];

    let ret_struct_ptr = syscall!(10004, fd, buf.as_mut_ptr());
    if ret_struct_ptr != 0 {
        return Vec::new();
    }

    buf
}

#[derive(Default)]
pub struct UserCommand {
    pub cmd: usize,
    pub offset: usize,
    pub buf_addr: usize,
    pub buf_size: usize,
    pub ok_signal: usize,
    pub ret_val: isize,
    pub ret_val2: isize,
    pub ret_val3: isize,
}

impl UserCommand {
    pub fn new(cmd: usize, offset: usize, buf_addr: usize, buf_size: usize) -> UserCommand {
        Self {
            cmd,
            offset,
            buf_addr,
            buf_size,
            ok_signal: 0,
            ret_val: -1,
            ret_val2: -1,
            ret_val3: -1,
        }
    }
}

impl Deref for UserCommand {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe {
            core::slice::from_raw_parts(self as *const UserCommand as *const u8, size_of::<Self>())
        }
    }
}

impl DerefMut for UserCommand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            core::slice::from_raw_parts_mut(self as *mut UserCommand as *mut u8, size_of::<Self>())
        }
    }
}

pub const USER_READ: usize = 1;
pub const USER_WRITE: usize = 2;
pub const USER_OPEN: usize = 3;
pub const USER_SIZE: usize = 4;
pub const USER_LIST: usize = 5;
pub const USER_IOCTL: usize = 6;

pub fn registfs(fs_name: &str, fs_addr: usize) -> isize {
    syscall!(10003, fs_name.as_ptr() as usize, fs_name.len(), fs_addr)
}

pub fn load_driver(driver_name: &str) -> isize {
    syscall!(10007, driver_name.as_ptr() as usize, driver_name.len())
}
