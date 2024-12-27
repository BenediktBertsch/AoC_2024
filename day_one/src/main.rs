use std::fs::read_to_string;
fn main() {
    let content = match read_file("./input".to_string()) {
        Ok(content) => content,
        Err(err) => panic!("{}", err),
    };
    let res = split_input_into_two_lists(content);
    let mut line_1 = res.0;
    let mut line_2 = res.1;
    line_1.sort();
    line_2.sort();
    // check that both lists have the same length
    if line_1.len() == line_2.len() {
        let total_distance = calculate_total_distance_between_two_lists(&line_1, &line_2);
        println!("Part one: {}", total_distance);
        let similarity_score = calculate_similarity_score(&line_1, &line_2);
        println!("Part two: {}", similarity_score);
    } else {
        panic!("The list is not correct in format, each left and should have an right entry and so on.");
    }
}

fn read_file(file_path: String) -> Result<String, std::io::Error> {
    let content_res = read_to_string(file_path);
    return content_res;
}

fn split_input_into_two_lists(file_content: String) -> (Vec<u32>, Vec<u32>) {
    let mut list_1 = Vec::<u32>::new();
    let mut list_2 = Vec::<u32>::new();
    for line in file_content.split("\n") {
        if line.chars().count() > 0 {
            let splitted_parts = line.split("   ").collect::<Vec<&str>>();
            list_1.push(splitted_parts[0].parse::<u32>().unwrap());
            list_2.push(splitted_parts[1].parse::<u32>().unwrap());
        }
    }
    return (list_1, list_2);
}

// part 1
fn calculate_total_distance_between_two_lists(list_one: &Vec<u32>, list_two: &Vec<u32>) -> u32 {
    let mut absolute_distance = 0;
    list_one.iter().zip(list_two.iter()).for_each(|(&a,&b)| absolute_distance += a.abs_diff(b));
    absolute_distance
}

// part two
fn calculate_similarity_score(list_one: &Vec<u32>, list_two: &Vec<u32>) -> u32 {
    let mut similarity_score = 0;
    for list_item_one in list_one.iter(){
        let mut counter = 0;
        for list_item_two in list_two.iter(){
            if list_item_one == list_item_two {
                counter += 1;
            };
        };
        similarity_score += list_item_one * counter;
    };
    similarity_score
}