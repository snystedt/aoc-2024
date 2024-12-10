use aoc_2024::input::read_lines;
use itertools::Itertools;

#[allow(unused)]
fn print_data(data: &[u16]) {
    println!(
        "{}",
        data.iter()
            .map(|b| if b == &u16::MAX {
                ".".to_string()
            } else {
                format!("[{}]", (*b).to_string())
            })
            .join("")
    );
}

fn first_star(mut data: Vec<u16>) {
    let mut file_block_ptr = data.len() - 1;
    let mut free_space_ptr = 0;

    while free_space_ptr < file_block_ptr {
        while data[free_space_ptr] != u16::MAX {
            free_space_ptr += 1;
        }

        while data[file_block_ptr] == u16::MAX {
            file_block_ptr -= 1;
        }

        if free_space_ptr >= file_block_ptr {
            break;
        }

        data[free_space_ptr] = data[file_block_ptr];
        data[file_block_ptr] = u16::MAX;
    }

    let checksum = data
        .iter()
        .enumerate()
        .filter(|(_, &b)| b != u16::MAX)
        .map(|(i, &b)| i * b as usize)
        .sum::<usize>();

    println!("First star checksum = {}", checksum);
}

fn second_star(mut data: Vec<u16>) {
    let mut file_block_ptr = data.len() - 1;

    let mut free_space_min = {
        let mut i = 0;
        while data[i] != u16::MAX {
            i += 1;
        }
        i
    };

    while file_block_ptr > 0 {
        let (file_id, file_block_end) = {
            // Find next file
            while data[file_block_ptr] == u16::MAX {
                file_block_ptr -= 1;
            }

            let file_id = data[file_block_ptr];
            let file_block_end = file_block_ptr + 1;
            while data[file_block_ptr] == file_id && file_block_ptr > 0 {
                file_block_ptr -= 1;
            }

            file_block_ptr += 1;

            (file_id, file_block_end)
        };

        let mut free_space_ptr = free_space_min;
        let mut free_space_end = free_space_min;
        while free_space_end - free_space_ptr < file_block_end - file_block_ptr
            && free_space_ptr < file_block_ptr
        {
            free_space_ptr = free_space_end;
            while data[free_space_ptr] != u16::MAX {
                free_space_ptr += 1;
            }

            free_space_end = free_space_ptr + 1;
            while free_space_end < data.len() && data[free_space_end] == u16::MAX {
                free_space_end += 1;
            }
        }

        if free_space_ptr < file_block_ptr
            && free_space_end - free_space_ptr >= file_block_end - file_block_ptr
        {
            if free_space_ptr == free_space_min {
                free_space_min += file_block_end - file_block_ptr;
            }
            data[free_space_ptr..free_space_ptr + (file_block_end - file_block_ptr)].fill(file_id);
            data[file_block_ptr..file_block_end].fill(u16::MAX);
        }

        file_block_ptr -= 1;
    }

    let checksum = data
        .iter()
        .enumerate()
        .filter(|(_, &b)| b != u16::MAX)
        .map(|(i, &b)| i * b as usize)
        .sum::<usize>();

    println!("Second star checksum = {}", checksum);
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day9/input.txt") {
        // Parsing
        let input = lines.flatten().collect::<Vec<_>>();
        assert!(input.len() == 1);

        let input = input[0].as_bytes().iter().map(|&b| (b - 48)).collect_vec();

        let data_len = input.iter().map(|&b| b as usize).sum::<usize>();
        let mut data = vec![u16::MAX; data_len];

        let mut ptr: usize = 0;
        let mut id: u16 = 0;
        let mut input_idx = 0;

        while input_idx < input.len() {
            for data_idx in ptr..ptr + input[input_idx] as usize {
                data[data_idx] = id;
            }
            ptr += input[input_idx] as usize;

            id += 1;
            input_idx += 1;
            if input_idx < input.len() {
                ptr += input[input_idx] as usize;
                input_idx += 1;
            }
        }

        first_star(data.clone());
        second_star(data);
    }
}
