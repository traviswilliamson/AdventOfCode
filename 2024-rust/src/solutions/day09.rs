use std::{cmp::Reverse, collections::BinaryHeap, error::Error};

const SUM_UP_TO_N: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let disk: Vec<usize> = _input.trim().bytes().map(|b| (b - b'0') as usize).collect();
    let mut first_free_index = 1;
    let mut last_file_index = disk.len() - 2 + disk.len() % 2;
    let mut moving_file_size = disk[last_file_index];
    let mut checksum = 0;
    let mut disk_head = 0;
    while first_free_index < last_file_index {
        // Add file we aren't moving
        let unmoved_file_size = disk[first_free_index - 1];
        checksum += ((first_free_index - 1) / 2) * (unmoved_file_size * disk_head + SUM_UP_TO_N[unmoved_file_size]);
        disk_head += unmoved_file_size;

        // Move files at the back up
        let mut open_space_size = disk[first_free_index];
        first_free_index += 2;
        while open_space_size > 0 {
            // Done with last file?
            if moving_file_size == 0 {
                if first_free_index == last_file_index {
                    // Don't bother, time to go home
                    break;
                }
                // On to the next!
                last_file_index -= 2;
                moving_file_size = disk[last_file_index];
            }

            // Move into open space
            let taken_space = moving_file_size.min(open_space_size);
            checksum += (last_file_index / 2) * (taken_space * disk_head + SUM_UP_TO_N[taken_space]);
            disk_head += taken_space;
            open_space_size -= taken_space;
            moving_file_size -= taken_space;
        }
    }

    // Anything left in the last file we partially moved?
    checksum += (last_file_index / 2) * (moving_file_size * disk_head + SUM_UP_TO_N[moving_file_size]);
    Ok(checksum.to_string())
}

struct Block {size: usize, disk_index: usize}
pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut disk: Vec<Block> = _input.trim().bytes().map(|b| Block {size: (b - b'0') as usize, disk_index: 0}).collect();
    for i in 1..disk.len() {
        disk[i].disk_index = disk[i-1].size + disk[i-1].disk_index;
    }
    let mut empty_spots: Vec<BinaryHeap<_>> = (0..10).map(|_| BinaryHeap::with_capacity(100)).collect();
    disk.iter().skip(1).step_by(2).for_each(|block| {
        empty_spots[block.size].push(Reverse(block.disk_index));
    });

    let mut checksum = 0;
    // Compute checksum, working through files
    // disk.iter().rev().skip((disk.len() + 1) % 2).step_by(2).for_each(| block| {
    //     // Find earliest space where the file will fit
    //     let (empty_spot_size, move_to_index) = empty_spots[block.size..10].iter().enumerate().map(|(i, h)| (i, h.peek())).filter(|(i, disk_index)| disk_index.is_some()).min_by_key(|s| s.1.unwrap());

    //     // If that's earlier than the file's position, move

    //     // Compute checksum, given the file's location
    //     checksum += (block.disk_index / 2) * (block.size * block.into() + SUM_UP_TO_N[block.size]);

    // });

    Ok(checksum.to_string())
}
