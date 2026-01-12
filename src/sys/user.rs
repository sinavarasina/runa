use libc::{gid_t, uid_t};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Uid(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gid(u32);

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
