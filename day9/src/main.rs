use anyhow::Result;

use std::{
    fs,
    io::{BufReader, Read},
};

fn main() -> Result<()> {
    // Read Input
    // let input_file = fs::File::open("puzzle_input_example.txt")?;
    let input_file = fs::File::open("puzzle_input.txt")?;
    let mut disk_map = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut disk_map)?;

    // First Compression Method
    let mut blocks = convert_disk_map_to_blocks(disk_map.clone());
    let mut first_space = blocks.iter().position(|x| x.is_empty());
    let mut last_file = blocks.iter().rposition(|x| !x.is_empty());
    while first_space.get_or_insert(blocks.len()) < last_file.get_or_insert(0) {
        // swap blocks
        if let Some(block_position1) = first_space {
            if let Some(block_position2) = last_file {
                let val1 = blocks[block_position1].clone();
                blocks[block_position1] = blocks[block_position2].clone();
                blocks[block_position2] = val1;
            }
        }
        // update while loop parameters
        first_space = blocks.iter().position(|x| x.is_empty());
        last_file = blocks.iter().rposition(|x| !x.is_empty());
    }
    let checksum = compute_checksum(&blocks);
    println!("First Method Checksum: {}", checksum);

    // Second Compression Method
    let mut file_system = convert_disk_map_to_files(disk_map);
    let reverse_files = file_system
        .clone()
        .into_iter()
        .filter(|x| !x.is_empty())
        .rev()
        .collect::<Vec<File>>();

    for file in reverse_files {
        let file_size = file.size;
        let file_current_index = file_system
            .iter()
            .position(|f| f == &file)
            .expect("Could not find file in filesystem");
        if let Some(first_open_space_with_size) = file_system
            .clone()
            .iter()
            .position(|f| f.is_empty() && f.size >= file_size)
        {
            if first_open_space_with_size < file_current_index {
                let extra_space = file_system[first_open_space_with_size].size - file.size;
                // remove the files that we are updating (back to front)
                let _ = file_system.remove(file_current_index);
                let _ = file_system.remove(first_open_space_with_size);
                // and put new files in their place (front to back)
                file_system.insert(first_open_space_with_size, file);
                file_system.insert(file_current_index, File::empty_space(file_size));
                // if there is extra space where we inserted the file, we need to
                // add a new empty space section with the extra space
                if extra_space > 0 {
                    file_system.insert(
                        first_open_space_with_size + 1,
                        File::empty_space(extra_space),
                    );
                }
            }
        }
    }
    let blocks2 = convert_filesystem_to_blocks(&file_system);
    let checksum2 = compute_checksum(&blocks2);
    println!("Second Method Checksum: {}", checksum2);

    Ok(())
}

fn convert_disk_map_to_blocks(disk_map: String) -> Vec<Block> {
    let mut is_file = true;
    let mut current_file_id = 0;
    let mut blocks: Vec<Block> = Vec::new();
    for block_char_num in disk_map.chars() {
        if block_char_num.is_numeric() {
            let block_count = block_char_num
                .to_string()
                .parse::<usize>()
                .expect("Unable to parse block size");
            // Push blocks for the size of the section
            for _ in 0..block_count {
                if is_file {
                    blocks.push(Block::new_with_id(current_file_id));
                } else {
                    blocks.push(Block::empty());
                }
            }
            // Increment file_id
            if is_file {
                current_file_id += 1;
            }
            // Switch block type
            is_file = !is_file;
        }
    }
    blocks
}

fn convert_disk_map_to_files(disk_map: String) -> Vec<File> {
    let mut is_file = true;
    let mut current_file_id = 0;
    let mut files: Vec<File> = Vec::new();
    for files_char_num in disk_map.chars() {
        if files_char_num.is_numeric() {
            let block_count = files_char_num
                .to_string()
                .parse::<usize>()
                .expect("Unable to parse block size");
            if is_file {
                files.push(File::new(current_file_id, block_count));
                // Increment file_id
                current_file_id += 1;
            } else {
                files.push(File::empty_space(block_count));
            }
            // Switch block type
            is_file = !is_file;
        }
    }
    files
}

fn convert_filesystem_to_blocks(filesystem: &[File]) -> Vec<Block> {
    let mut blocks = Vec::new();
    for file in filesystem {
        if file.is_empty() {
            for _ in 0..file.size {
                blocks.push(Block::empty());
            }
        } else {
            for _ in 0..file.size {
                blocks.push(Block::new_with_id(
                    file.file_id_number.expect("non-empty file has no size!"),
                ))
            }
        }
    }
    blocks
}

fn compute_checksum(blocks: &[Block]) -> usize {
    blocks.iter().enumerate().fold(0, |checksum, (i, block)| {
        if let Some(file_id) = block.file_id_number {
            checksum + (i * file_id)
        } else {
            checksum
        }
    })
}

#[derive(Clone)]
struct Block {
    file_id_number: Option<usize>,
}

impl Block {
    fn new_with_id(id: usize) -> Block {
        Block {
            file_id_number: Some(id),
        }
    }

    fn empty() -> Block {
        Block {
            file_id_number: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.file_id_number.is_none()
    }
}

#[derive(Clone, PartialEq)]
struct File {
    file_id_number: Option<usize>,
    size: usize,
}

impl File {
    fn new(id: usize, size: usize) -> File {
        File {
            file_id_number: Some(id),
            size,
        }
    }

    fn empty_space(size: usize) -> File {
        File {
            file_id_number: None,
            size,
        }
    }

    fn is_empty(&self) -> bool {
        self.file_id_number.is_none()
    }
}
