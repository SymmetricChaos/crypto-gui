pub(crate) const BLOCK_BYTES: usize = 1024;
pub(crate) const BLOCK_WORDS: usize = BLOCK_BYTES / 8;

pub(crate) const MIN_PAR: u32 = 1;
pub(crate) const MAX_PAR: u32 = 1 << 24;

pub(crate) const MAX_PASS: usize = 0xffffffff;
pub(crate) const MIN_SALT: usize = 0x08;
pub(crate) const MAX_SALT: usize = 0xffffffff;
pub(crate) const MAX_KEY: usize = 0xffffffff;

// Number of synchronization points between lanes per pass
// TODO: Is this S, the number of slices, in the spec?
pub(crate) const SYNC_POINTS: usize = 4;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mode {
    I,
    D,
    ID,
}

impl Mode {
    pub fn to_u64(&self) -> u64 {
        match self {
            Mode::I => 0,
            Mode::D => 1,
            Mode::ID => 2,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Mode::I => 0,
            Mode::D => 1,
            Mode::ID => 2,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Version {
    V10,
    V13,
}

impl Version {
    pub fn to_u64(&self) -> u64 {
        match self {
            Version::V10 => 0x10,
            Version::V13 => 0x13,
        }
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Version::V10 => 0x10,
            Version::V13 => 0x13,
        }
    }
}
