use std::convert::TryInto;

use anyhow::Result;

pub fn decompress_part_file(raw: &[u8]) -> Result<Vec<u8>> {
    let final_size = get_total_original_size(raw)?;
    let mut decompressed = vec![0; final_size];

    let mut compressed_cursor = 0;
    let mut decompressed_cursor = 0;

    while compressed_cursor < raw.len() {
        let compressed_block_size: usize =
            u32::from_le_bytes(raw[compressed_cursor..(compressed_cursor + 4)].try_into()?)
                .try_into()?;
        let original_block_size: usize =
            u32::from_le_bytes(raw[(compressed_cursor + 4)..(compressed_cursor + 8)].try_into()?)
                .try_into()?;
        if original_block_size == 0 {
            break;
        }
        let block = &raw[(compressed_cursor + 8)..(compressed_cursor + 4 + compressed_block_size)];
        let destination_subslice =
            &mut decompressed[decompressed_cursor..(decompressed_cursor + original_block_size)];
        let _ = lzzzz::lz4::decompress_partial(block, destination_subslice, original_block_size)?;

        compressed_cursor += compressed_block_size + 4;
        decompressed_cursor += original_block_size;
    }

    Ok(decompressed)
}

fn get_total_original_size(raw: &[u8]) -> Result<usize> {
    let mut total = 0;
    let mut cursor = 0;

    while cursor < raw.len() {
        let block_size: usize =
            u32::from_le_bytes(raw[cursor..(cursor + 4)].try_into()?).try_into()?;
        let original_size: usize =
            u32::from_le_bytes(raw[(cursor + 4)..(cursor + 8)].try_into()?).try_into()?;
        total += original_size;
        cursor += block_size + 4
    }

    Ok(total)
}
