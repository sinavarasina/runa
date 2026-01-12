use libc::{gid_t, uid_t};
use std::{ffi::CStr, io};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uid(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gid(u32);

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub uid: Uid,
    pub gid: Gid,
    pub shell: String,
    pub dir: String,
}

impl Uid {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_raw(&self) -> uid_t {
        self.0 as uid_t
    }

    pub fn is_root(&self) -> bool {
        self.0 == 0
    }
}

impl Gid {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn as_raw(&self) -> gid_t {
        self.0 as gid_t
    }
}

pub fn get_uid() -> Uid {
    let raw = unsafe { libc::getuid() };
    Uid(raw as u32)
}

pub fn get_gid() -> Gid {
    let raw = unsafe { libc::getgid() };

    Gid(raw)
}

pub fn get_effective_uid() -> Uid {
    let raw = unsafe { libc::geteuid() };
    Uid(raw)
}

unsafe fn c_str_to_string(ptr: *const libc::c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

unsafe fn passwd_to_user(pw: libc::passwd) -> User {
    User {
        name: c_str_to_string(pw.pw_name),
        uid: Uid(pw.pw_uid as u32),
        gid: Gid(pw.pw_gid as u32),
        shell: c_str_to_string(pw.pw_shell),
        dir: c_str_to_string(pw.pw_dir),
    }
}
pub fn get_user_by_uid(uid: Uid) -> io::Result<User> {
    unsafe {
        let pw_ptr = libc::getpwuid(uid.as_raw());

        if pw_ptr.is_null() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("user with uid {:?} was not found", uid),
            ));
        }
        Ok(passwd_to_user(*pw_ptr))
    }
}

pub fn get_user_by_name(name: &str) -> io::Result<User> {
    let c_name = std::ffi::CString::new(name)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "username contain null bytes"))?;
    unsafe {
        let pw_ptr = libc::getpwnam(c_name.as_ptr());

        if pw_ptr.is_null() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("username {} was not found", name),
            ));
        }
        Ok(passwd_to_user(*pw_ptr))
    }
}
