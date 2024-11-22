#![allow(non_camel_case_types)]

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SdkAppTypes {
    desc_NotSpecified = 0,
    desc_Application,
    desc_DlpChild,
    desc_Demo,
    desc_EcApplication, // integrated in (Ext)Application since SDK 7
    desc_ExtApplication, // Snake equivalent of desc_Application (128MB/804MHz/L2 Cache)
    desc_ExtDlpChild,    // Snake equivalent of desc_DlpChild (128MB/804MHz/L2 Cache)
    desc_ExtDemo,        // Snake equivalent of desc_Demo (128MB/804MHz/L2 Cache)
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CtrSdkDepList {
    pub fw_minor: u32,
    pub dependency: [u8; 0x180],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CtrSdkDesc {
    pub type_: u32,
    pub fw_minor: u32,
    pub exheader_desc: [u8; 0x200],
    pub signed_desc: [u8; 0x200],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CtrSdkDescSignData {
    pub type_: u32,
    pub fw_minor: u32,
    pub modulus: [u8; 0x100],
    pub priv_exponent: [u8; 0x100],
    pub access_desc_signature: [u8; 0x100],
}
