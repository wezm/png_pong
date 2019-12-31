//! PNG chunks                                                             / */
pub fn lodepng_chunk_length(chunk: &[u8]) -> usize {
    super::lodepng_read32bit_int(chunk) as usize
}

pub fn lodepng_chunk_type(chunk: &[u8]) -> &[u8] {
    &chunk[4..8]
}

pub(crate) fn lodepng_chunk_data(chunk: &[u8]) -> Result<&[u8], super::Error> {
    let len = lodepng_chunk_length(chunk) as usize;
    /*error: chunk length larger than the max PNG chunk size*/
    if len > (1 << 31) {
        return Err(super::Error(63));
    }
    if chunk.len() < len + 12 {
        return Err(super::Error(64));
    }

    Ok(&chunk[8..8 + len])
}

pub(crate) fn lodepng_chunk_data_mut(
    chunk: &mut [u8],
) -> Result<&mut [u8], super::Error> {
    let len = lodepng_chunk_length(chunk) as usize;
    /*error: chunk length larger than the max PNG chunk size*/
    if len > (1 << 31) {
        return Err(super::Error(63));
    }
    if chunk.len() < len + 12 {
        return Err(super::Error(64));
    }

    Ok(&mut chunk[8..8 + len])
}

pub(crate) fn lodepng_chunk_next(chunk: &[u8]) -> &[u8] {
    let total_chunk_length = lodepng_chunk_length(chunk) as usize + 12;
    &chunk[total_chunk_length..]
}

pub fn lodepng_chunk_ancillary(chunk: &[u8]) -> bool {
    (chunk[4] & 32) != 0
}

pub fn lodepng_chunk_private(chunk: &[u8]) -> bool {
    (chunk[6] & 32) != 0
}

pub fn lodepng_chunk_safetocopy(chunk: &[u8]) -> bool {
    (chunk[7] & 32) != 0
}

#[cfg(not(fuzzing))]
pub fn lodepng_chunk_check_crc(chunk: &[u8]) -> bool {
    let length = lodepng_chunk_length(chunk) as usize;
    /*the CRC is taken of the data and the 4 chunk type letters, not the length*/
    let crc = super::lodepng_read32bit_int(&chunk[length + 8..]);
    let checksum = super::lodepng_crc32(&chunk[4..length + 8]);
    crc == checksum
}

#[cfg(fuzzing)]
pub fn lodepng_chunk_check_crc(chunk: &[u8]) -> bool {
    true // Disable crc32 checks so that random data from fuzzer gets actually parsed
}

pub fn lodepng_chunk_generate_crc(chunk: &mut [u8]) {
    let length = lodepng_chunk_length(chunk) as usize;
    let crc = super::lodepng_crc32(&chunk[4..length + 8]);
    super::lodepng_set32bit_int(&mut chunk[8 + length..], crc);
}

pub(crate) fn chunk_append(out: &mut Vec<u8>, chunk: &[u8]) {
    let total_chunk_length = lodepng_chunk_length(chunk) as usize + 12;
    out.extend_from_slice(&chunk[0..total_chunk_length]);
}