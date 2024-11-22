#![allow(non_camel_case_types)]

#[repr(C)]
pub struct crr_hdr {
    magic: [u8; 4],
    reserved0: [u8; 4],
    node0: [u8; 4],
    node1: [u8; 4],
    debug_info_offset: [u8; 4],  // s32
    debug_info_size: [u8; 4],    // s32
    reserved1: [u8; 8],
    unique_id_mask: [u8; 4],
    unique_id_pattern: [u8; 4],
    reserved2: [u8; 0x18],
    sign_public_key: [u8; 0x100],
    sign_public_key_sign: [u8; 0x100],
    sign: [u8; 0x100],
    unique_id: [u8; 4],
    size: [u8; 4],
    reserved3: [u8; 8],
    hash_offset: [u8; 4],
    num_hash: [u8; 4],
    module_id_offset: [u8; 4],
    module_id_size: [u8; 4],
}
