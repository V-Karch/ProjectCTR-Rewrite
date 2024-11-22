#![allow(non_camel_case_types)]

// Bools
pub enum ReturnBasic {
    Good,
    Fail,
}

pub enum GlobalErrors {
    MemError = -1,
    FailedToOpenFile = -2,
    FailedToImportFile = -3,
    FailedToCreateOutfile = -4,
}

pub enum EndiannessFlag {
    BE = 0,
    LE = 1,
}

pub enum FileUnitSize {
    KB = 1024,
    MB = 1048576,
    GB = 1073741824,
}

#[repr(u64)]  // Force the enum to use u64 internally
pub enum DataTypeMax {
    MaxU8 = 0xff,
    MaxU16 = 0xffff,
    MaxU32 = 0xffffffff,
    MaxU64 = 0xffffffffffffffff,
}

// Type aliases

pub type s8 = i8;
pub type s16 = i16;
pub type s32 = i32;
pub type s64 = i64;
