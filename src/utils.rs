use std::fs::{metadata, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

const IO_BLOCKSIZE: usize = 5 * 1024 * 1024; // 5 MB

// Helper for copying data
fn copy_data(dest: &mut Option<Vec<u8>>, source: &[u8]) -> i32 {
    if dest.is_none() {
        *dest = Some(Vec::with_capacity(source.len()));
    }
    if let Some(ref mut vec) = dest {
        vec.copy_from_slice(source);
    }
    0
}

// Random byte fill function
fn rndset(ptr: &mut [u8], num: usize) {
    for i in 0..num {
        ptr[i] = u8_get_rand();
    }
}

// Clears memory
fn clrmem(ptr: &mut [u8]) {
    ptr.fill(0);
}

// Round up value to next alignment
fn roundup(value: u64, alignment: u64) -> u64 {
    value + alignment - value % alignment
}

// Aligns value to a specified alignment
fn align(value: u64, alignment: u64) -> u64 {
    if value % alignment != 0 {
        roundup(value, alignment)
    } else {
        value
    }
}

// Get minimum of two u64 values
fn min64(a: u64, b: u64) -> u64 {
    if a < b {
        a
    } else {
        b
    }
}

// Get maximum of two u64 values
fn max64(a: u64, b: u64) -> u64 {
    if a > b {
        a
    } else {
        b
    }
}

// Replace file extension
fn replace_filextension(input: &str, new_ext: &str) -> Option<String> {
    let ext_pos = input.rfind('.');
    match ext_pos {
        Some(pos) => Some(format!("{}{}", &input[..pos], new_ext)),
        None => Some(format!("{}{}", input, new_ext)),
    }
}

// Dump memory to a file
fn memdump(fout: &mut File, prefix: &str, data: &[u8], mut size: usize) {
    let mut offset = 0;
    let mut line = 0;
    let prefixlen = prefix.len();

    while size > 0 {
        let max = size.min(32);
        if line == 0 {
            fout.write_all(prefix.as_bytes()).unwrap();
        } else {
            fout.write_all(&vec![' ' as u8; prefixlen]).unwrap();
        }

        for i in 0..max {
            write!(fout, "{:02X}", data[offset + i]).unwrap();
        }
        fout.write_all(b"\n").unwrap();

        offset += max;
        size -= max; // This is now valid because `size` is mutable
        line += 1;
    }
}

// Check if a character is a valid base64 character
fn is_valid_b64_char(chr: char) -> bool {
    chr.is_alphanumeric() || chr == '+' || chr == '/' || chr == '='
}

// Get the length of a valid base64 string
fn b64_strlen(str: &str) -> usize {
    str.chars().filter(|&chr| is_valid_b64_char(chr)).count()
}

// Copy a base64 string (ignoring invalid characters)
fn b64_strcpy(dst: &mut String, src: &str) {
    for chr in src.chars() {
        if is_valid_b64_char(chr) {
            dst.push(chr);
        }
    }
}

// Initialize random number generator
fn init_rand() {
    rand::thread_rng();
}

// Get random u8 value
fn u8_get_rand() -> u8 {
    rand::random::<u8>()
}

// Get random u16 value
fn u16_get_rand() -> u16 {
    rand::random::<u16>()
}

// Get random u32 value
fn u32_get_rand() -> u32 {
    rand::random::<u32>()
}

// Get random u64 value
fn u64_get_rand() -> u64 {
    rand::random::<u64>()
}

// Assert file exists
fn assert_file(filename: &str) -> bool {
    Path::new(filename).exists()
}

// Get file size
fn get_file_size64(filename: &str) -> u64 {
    match metadata(filename) {
        Ok(metadata) => metadata.len(),
        Err(_) => 0,
    }
}

// Make a directory
fn makedir(dir: &str) -> i32 {
    if let Err(_) = std::fs::create_dir_all(dir) {
        return -1;
    }
    0
}

// Get current working directory
fn get_cwdir() -> Option<String> {
    std::env::current_dir()
        .ok()
        .map(|p| p.to_str().unwrap_or_default().to_string())
}

// Truncate file to specified size
fn truncate_file64(filename: &str, filelen: u64) -> i32 {
    let file = File::open(filename);
    match file {
        Ok(f) => {
            if let Err(_) = f.set_len(filelen) {
                return -1;
            }
            0
        }
        Err(_) => -1,
    }
}

// Import file into memory
fn import_file(file: &str, size: u64) -> Option<Vec<u8>> {
    let fsize = get_file_size64(file);
    if size > 0 && size != fsize {
        eprintln!("[!] {} has an invalid size (0x{:X})", file, fsize);
        return None;
    }

    let mut data = vec![0u8; fsize as usize];
    let mut file = File::open(file).ok()?;
    file.read_exact(&mut data).ok()?;
    Some(data)
}

// Write buffer to file at specified offset
fn write_buffer(fout: &mut File, buffer: &[u8], size: u64, offset: u64) {
    fout.seek(SeekFrom::Start(offset)).unwrap(); // Use Seek trait

    let mut i: u64 = 0;
    while i < size {
        let block_size = IO_BLOCKSIZE.min((size - i) as usize); // Convert size to usize
        let end_index = (i + block_size as u64) as usize; // Ensure the addition is cast to usize after
        fout.write_all(&buffer[i as usize..end_index]).unwrap(); // Convert i to usize for indexing
        i += block_size as u64;
    }
}

// Read file into buffer from specified offset
fn read_file64(fout: &mut File, buffer: &mut [u8], size: u64, offset: u64) {
    fout.seek(SeekFrom::Start(offset)).unwrap();
    let mut i = 0;
    while i < size {
        let block_size = IO_BLOCKSIZE.min((size - i) as usize);
        let end_index = (i + block_size as u64) as usize; // Ensure the addition is cast to usize after
        fout.read_exact(&mut buffer[i as usize..end_index]).unwrap(); // Convert i to usize for indexing
        i += block_size as u64;
    }
}

// Convert u8 slice to u16 based on endianness
fn u8_to_u16(value: &[u8], endianness: u8) -> u16 {
    match endianness {
        0 => ((value[1] as u16) << 0) | ((value[0] as u16) << 8), // Cast u8 to u16 before shifting
        1 => ((value[0] as u16) << 0) | ((value[1] as u16) << 8), // Cast u8 to u16 before shifting
        _ => 0,
    }
}

// Convert u8 slice to u32 based on endianness
fn u8_to_u32(value: &[u8], endianness: u8) -> u32 {
    match endianness {
        0 => {
            ((value[3] as u32) << 0)
                | ((value[2] as u32) << 8)
                | ((value[1] as u32) << 16)
                | ((value[0] as u32) << 24)
        } // Cast u8 to u32 before shifting
        1 => {
            ((value[0] as u32) << 0)
                | ((value[1] as u32) << 8)
                | ((value[2] as u32) << 16)
                | ((value[3] as u32) << 24)
        } // Cast u8 to u32 before shifting
        _ => 0,
    }
}

// Convert u8 slice to u64 based on endianness
fn u8_to_u64(value: &[u8], endianness: u8) -> u64 {
    match endianness {
        0 => {
            (value[7] as u64) << 0
                | (value[6] as u64) << 8
                | (value[5] as u64) << 16
                | (value[4] as u64) << 24
                | (value[3] as u64) << 32
                | (value[2] as u64) << 40
                | (value[1] as u64) << 48
                | (value[0] as u64) << 56
        }
        1 => {
            (value[0] as u64) << 0
                | (value[1] as u64) << 8
                | (value[2] as u64) << 16
                | (value[3] as u64) << 24
                | (value[4] as u64) << 32
                | (value[5] as u64) << 40
                | (value[6] as u64) << 48
                | (value[7] as u64) << 56
        }
        _ => 0,
    }
}
