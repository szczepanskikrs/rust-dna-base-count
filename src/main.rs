use clap::Parser;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(version,about,long_about=None)]
struct CliArgs {
    #[arg(short, long)]
    sequence: String,
}
const SINGLE_BASE_COUNT: usize = 1;
const BASE_COUNT_CHUNK_SIZE: usize = 50;

fn main() {
    let dna_cli_args = CliArgs::parse();

    println!("Parsing DNA sequence");

    let sorted_base_count =
        dna_cli_args
            .sequence
            .chars()
            .fold(HashMap::<char, usize>::new(), |mut registry, base| {
                match registry.get(&base) {
                    None => registry.insert(base, SINGLE_BASE_COUNT),
                    Some(current_base_count) => {
                        registry.insert(base, *current_base_count + SINGLE_BASE_COUNT)
                    }
                };
                registry
            });

    println!("SEQUENCE:");

    dna_cli_args
        .sequence
        .chars()
        .into_iter()
        .chunks(BASE_COUNT_CHUNK_SIZE)
        .into_iter()
        .map(|mut chunk| println!("{}", chunk.join("")))
        .last();

    sorted_base_count
        .iter()
        .for_each(|(base, count)| println!("Count of: {} {}", base, count));

    println!("Total Count: {}", dna_cli_args.sequence.len())
}
