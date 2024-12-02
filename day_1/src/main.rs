use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let grouped_ids = fs::read_to_string(file_path).expect("Unable to read file");
 
    dbg!(calc_total_distance(&grouped_ids));
    dbg!(calc_similarity_score(&grouped_ids));
}

fn calc_total_distance( grouped_ids: &String ) -> i64 {
    let mut location_id_list_1: Vec<i64> = Vec::new();
    let mut location_id_list_2: Vec<i64> = Vec::new();
    
    let ids = grouped_ids.split_whitespace();
    let mut peekable_ids = ids.peekable();
    let id_parse_panic_msg = "Id should be of type i64";

    while peekable_ids.peek().is_some() {
        location_id_list_1.push(peekable_ids.next().unwrap().parse().expect(&id_parse_panic_msg));
        location_id_list_2.push(peekable_ids.next()
            .expect("Every id should have another id associated with it")
            .parse().expect(&id_parse_panic_msg));
    }

    location_id_list_1.sort();
    location_id_list_2.sort();

    let list_length = location_id_list_1.len();
    let mut total_distance : i64 = 0;
    for i in 0..list_length {
        total_distance += i64::abs( location_id_list_1[i] - location_id_list_2[i]);
    }
 
    total_distance
}

fn calc_similarity_score( grouped_ids: &String ) -> i64 {
    let mut location_id_list_1: Vec<i64> = Vec::new();
    let mut location_id_list_2: Vec<i64> = Vec::new();
    
    let ids = grouped_ids.split_whitespace();
    let mut peekable_ids = ids.peekable();
    let id_parse_panic_msg = "Id should be of type i64";

    while peekable_ids.peek().is_some() {
        location_id_list_1.push(peekable_ids.next().unwrap().parse().expect(&id_parse_panic_msg));
        location_id_list_2.push(peekable_ids.next()
            .expect("Every id should have another id associated with it")
            .parse().expect(&id_parse_panic_msg));
    }

    location_id_list_1.sort();
    location_id_list_2.sort();

    let mut list_2_index = 0;
    let mut prev_id = -1;
    let mut prev_matches = 0;
    let stopping_index = location_id_list_2.len() - 1;
    let mut sim_score = 0;

    for id in location_id_list_1 {
        // Check to use cached data
        if id == prev_id {
            sim_score += id * prev_matches;
            continue;
        }

        let mut location_id_2 = location_id_list_2[list_2_index];

        while id > location_id_2 {
            if list_2_index < stopping_index {
                list_2_index += 1;
                location_id_2 = location_id_list_2[list_2_index];
            }
            // Stopping point reached
            else {
                return sim_score;
            }
        }

        let mut sim_count = 0;
        while id == location_id_2 {
            sim_count += 1;
            list_2_index += 1;
            location_id_2 = location_id_list_2[list_2_index];
        }

        sim_score += sim_count * id;
        prev_id = id;
        prev_matches = sim_count;
    }

    sim_score
}
