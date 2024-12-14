use std::error::Error;

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

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    // let mut empty_spots: Vec<BinaryHeap<_>> = (0..10).map(|_| BinaryHeap::with_capacity(100)).collect();
    
    let mut empty_spots: Vec<_> = (0..10).map(|_| Vec::with_capacity(1_100)).collect();
    let disk: Vec<usize> = _input.trim().bytes().map(|b| (b - b'0') as usize).collect();
    let mut seek_head = 0;
    for (i, &size) in disk.iter().enumerate() {
        if i % 2 == 1 && size > 0 {
            empty_spots[size].push(seek_head);
        }

        seek_head += size;
    }

    // Turn empty spots into mins for performance on removes
    empty_spots.iter_mut().for_each(|v| {
        // Make sure they'll never be empty
        v.push(seek_head);
        v.reverse();
    });

    let mut checksum = 0;
    // Compute checksum, working through files
    for (i, &size) in disk.iter().enumerate().rev() {
        // Seek
        seek_head -= size;

        // Empty space? Done
        if i % 2 == 1 {
            continue;
        }

        // Find earliest space where the file will fit
        let mut move_to_index = seek_head;
        let mut move_to_size = usize::MAX;
        for free_i in size..empty_spots.len() {
            let end = empty_spots[free_i].len() - 1;
            let earliest_spot = empty_spots[free_i][end];

            if earliest_spot < move_to_index {
                move_to_index = earliest_spot;
                move_to_size = free_i;
            }
        }

        // Have we passed all the biggest empty spaces?
        // Then we don't care about them anymore
        // It's not like we'll be resizing any other spaces down into them, they're the biggest
        if !empty_spots.is_empty() {
            let largest_empty_spot = empty_spots.len() - 1;
            let end = empty_spots[largest_empty_spot].len() - 1;
            if empty_spots[largest_empty_spot][end] > seek_head {
                empty_spots.pop();
            }
        }

        // Compute checksum, given the file's location
        checksum += (i / 2) * (size * move_to_index + SUM_UP_TO_N[size]);

        // Did we move into an empty spot?
        if move_to_size != usize::MAX {
            // Then clear what we moved into
            empty_spots[move_to_size].pop();
            // And insert into the min-heap for the smaller space
            let now_available_size = move_to_size - size;
            if now_available_size > 0 {
                let now_available_index = move_to_index + size;
                // Start after the end, we may be inserting right at the back
                let mut empty_seek_index = empty_spots[now_available_size].len();
                // Seek backwards to put in the right spot
                // This is pretty close to the beginning of empty spaces, so we shouldn't have to seek far
                while now_available_index > empty_spots[now_available_size][empty_seek_index - 1] {
                    empty_seek_index -= 1;
                }

                empty_spots[now_available_size].insert(empty_seek_index, now_available_index);
            }
        }
    }

    Ok(checksum.to_string())
}
