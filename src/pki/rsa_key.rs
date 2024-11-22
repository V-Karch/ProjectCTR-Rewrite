#[repr(C)]
pub struct CtrRsa2048Key {
    pub modulus: [u8; 0x100],
    pub priv_exponent: [u8; 0x100],
}

#[repr(C)]
pub struct CtrRsa4096Key {
    pub modulus: [u8; 0x200],
    pub priv_exponent: [u8; 0x200],
}
