use std::{fs::read_to_string, str::SplitWhitespace};

fn main() {
    let file_content = read_content_from_file("./input".to_string());
    let all_lines_from_file_content = file_content.lines();
    let mut count_of_safe_levels = 0;
    for (i, line) in all_lines_from_file_content.enumerate() {
        if check_level_safety_with_one_removed(line) {
            count_of_safe_levels += 1;
        }
    }
    println!("Total of safe levels: {}", count_of_safe_levels);
}

fn check_level_safety_with_one_removed(line: &str) -> bool {
    let mut current_line_string = line.to_string();
    let res = level_is_safe(&current_line_string.split_whitespace());
    if res.0 {
        return true;
    }
    let original_line = current_line_string.clone();
    for i in 0..current_line_string.split_whitespace().count() {
        let filtered_line = original_line
          .split_whitespace()
          .enumerate()
          .filter(|&(index, _)| index != i)
          .map(|(_, val)| val)
          .collect::<Vec<&str>>().join(" ");
        current_line_string = filtered_line;
        if level_is_safe(&current_line_string.split_whitespace()).0 {
            return true;
        }
    }
    false
}

fn level_is_safe(level_splitted: &SplitWhitespace) -> (bool, u8) {
    let mut decrease = false;
    let mut increase = false;
    let mut bad_jump_count: u8 = 0;
    let parsed_level: Vec<i8> = level_splitted
        .clone()
        .map(|x| x.parse::<i8>().unwrap())
        .collect();
    // println!("{:?}", parsed_level);
    for (i, elem) in parsed_level.iter().enumerate() {
        if i + 1 < parsed_level.len() {
            bad_jump_count = i as u8;
            // println!("elem: {}, i: {}, next value: {}", elem, i, parsed_level[i+1]);
            if elem < &parsed_level[i + 1] {
                increase = true;
            }

            if elem - &parsed_level[i + 1] > 0 {
                decrease = true;
            }

            // Check that jumps has to be at least 1
            if elem - &parsed_level[i + 1] == 0 {
                // println!(
                //     "bad jump found: {} to {}, bad before adding this one counter: {}",
                //     elem,
                //     &parsed_level[i + 1],
                //     bad_jump_count
                // );
                return (false, bad_jump_count);
            }

            // Check that only increase or decrease of jumps on one level is allowed
            if decrease && increase {
                // println!(
                //     "bad jump found: {} to {}, bad before adding this one counter: {}",
                //     elem,
                //     &parsed_level[i + 1],
                //     bad_jump_count
                // );
                return (false, bad_jump_count);
            }

            // Check that if positive max increase/decrease of 3
            if elem.abs_diff(parsed_level[i + 1]) > 3 {
                // println!(
                //     "bad jump found: {} to {}, bad before adding this one counter: {}",
                //     elem,
                //     &parsed_level[i + 1],
                //     bad_jump_count
                // );
                return (false, bad_jump_count);
            }
        }
    }
    (true, bad_jump_count)
}

fn read_content_from_file(filepath: String) -> String {
    let content = read_to_string(filepath).unwrap();
    content
}
