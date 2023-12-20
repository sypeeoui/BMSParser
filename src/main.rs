// ProgettoBMS.parsechart riscritto in rust 
// Suqi Chen

// use std::io;
use std::fs;
use std::env;

mod parsechart;
use parsechart::{process_chart_info, process_main_data, sort_and_group_notes, generate_output_file};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file;
    let output_file;

    if args.len() == 1 {
        println!("Please specify the input file");
        return;
    } else if args.len() == 2 {
        input_file = args[1].to_string();
        let index = input_file.rfind('.').unwrap();
        output_file = input_file[..index].to_string() + ".json";
    } else if args.len() == 3 {
        input_file = args[1].to_string();
        output_file = args[2].to_string();
    } else {
        println!("Too many arguments");
        return;
    }

    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");

    let mut lines = contents.lines();
    let (bpms, stops, chart) = process_chart_info(&mut lines);
    let (mut bars, mut barelements) = process_main_data(&mut lines, &chart, &bpms, &stops);

    sort_and_group_notes(&mut bars, &mut barelements);

    generate_output_file(&chart, &bars, &output_file);
}