use rand::prelude::*;
use std::{cmp::Ordering, fs};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Params {
    #[structopt(
        short = "p",
        long,
        help = "Path to raw grayscale image 8-bits per pixel"
    )]
    path: String,

    #[structopt(short = "w", help = "Image width in pixels")]
    image_width: u32,

    #[structopt(short = "h", help = "Image height in pixels")]
    image_height: u32,

    #[structopt(
        short = "r",
        help = "Specifies after what cycle to print",
        default_value = "100"
    )]
    print_rate: u32,

    #[structopt(short = "c", long, help = "Number of cycles")]
    cycles: u32,

    #[structopt(short, long, help = "Where to store results", default_value = "./out/")]
    output: String,
}

fn main() {
    let params: Params = Params::from_args();

    let CYCLES: u32 = params.cycles;
    let WIDTH: u32 = params.image_width;
    let HEIGHT: u32 = params.image_height;

    const POPULATION_SIZE: usize = 300;
    let BEST_NUMBER: u8 = 10;

    let original_bytes = fs::read(params.path).expect("Failed to read bytes");

    let mut population: Vec<Vec<u8>> = Vec::with_capacity(POPULATION_SIZE);
    let mut best: Vec<Vec<u8>> = vec![vec![0; original_bytes.len()]; BEST_NUMBER as usize];

    let mut scores: [(usize, usize); POPULATION_SIZE] = [(0, 0); POPULATION_SIZE];

    for i in 0..POPULATION_SIZE {
        population.push(vec![0; original_bytes.len()]);
        scores[i].0 = i;
    }

    for i in 0..CYCLES {
        cross(&mut population, &best);
        mutate(&mut population, HEIGHT, WIDTH);
        score(&population, &original_bytes, &mut scores);
        note_best_speciment(&population, &mut scores, &mut best);

        if i % params.print_rate == 0 {
            println!("Cycle {}", i);
            fs::write(format!("./{}/{}_best.raw", params.output, i), &best[0])
                .expect(&format!("Failed to write best in cycle: {}", i));
        }
    }
    fs::write(format!("./{}/best.raw", params.output), &best[0])
        .expect("Failed to save final result");
}

fn note_best_speciment<const LENGTH: usize>(
    population: &Vec<Vec<u8>>,
    scores: &mut [(usize, usize); LENGTH],
    best: &mut Vec<Vec<u8>>,
) {
    scores.sort_by(|a, b| {
        if a.1 < b.1 {
            return Ordering::Less;
        }

        if a.1 > b.1 {
            return Ordering::Greater;
        }

        Ordering::Equal
    });

    for i in 0..best.len() - 1 {
        best[i] = population[scores[i].0].clone();
    }
}

fn cross(population: &mut Vec<Vec<u8>>, best: &Vec<Vec<u8>>) {
    let mut rng = rand::thread_rng();

    for i in 0..population.len() {
        // let random_speciment = population.choose_mut(&mut rng).unwrap();
        let random_speciment = &mut population[i];
        let random_best_speciment = best.choose(&mut rng).unwrap();

        for i in 0..random_speciment.len() - 1 {
            random_speciment[i] = random_best_speciment[i];
        }
    }
}

fn mutate(population: &mut Vec<Vec<u8>>, HEIGHT: u32, WIDTH: u32) {
    let mut rng = rand::thread_rng();
    for speciment in population {
        let end_x: u32 = rng.gen_range(1..WIDTH - 1);
        let end_y: u32 = rng.gen_range(1..HEIGHT - 1);

        let start_x: u32 = rng.gen_range(1..WIDTH - end_x);
        let start_y: u32 = rng.gen_range(1..HEIGHT - end_y);

        let color: u8 = rng.gen_range(0..255);

        for x in end_y..end_y + start_y {
            for y in end_x..end_x + start_x {
                let index = (x * WIDTH + y) as usize;
                speciment[index] = (speciment[index] / 2) + (color / 2);
            }
        }
    }
}

fn score<const LENGTH: usize>(
    population: &Vec<Vec<u8>>,
    original_bytes: &Vec<u8>,
    scores: &mut [(usize, usize); LENGTH],
) {
    for j in 0..population.len() - 1 {
        let mut value: usize = 0;
        for i in 0..original_bytes.len() - 1 {
            value += match original_bytes[i] < population[j][i] {
                true => (population[j][i] - original_bytes[i]) as usize,
                false => (original_bytes[i] - population[j][i]) as usize,
            };
        }
        scores[j].1 = value;
        scores[j].0 = j;
    }
}
