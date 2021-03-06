use rand::prelude::*;
use std::{cmp::Ordering, fs};
use structopt::StructOpt;
use minifb::{Key, Window, WindowOptions};

mod evaluator;
pub mod genetic;

extern crate minifb;

#[derive(StructOpt, Debug)]
struct Params {
    #[structopt(
        short = "p",
        long,
        help = "Path to raw grayscale image 8-bits per pixel"
    )]
    path: String,

    #[structopt(short = "w", help = "Image width in pixels")]
    image_width: usize,

    #[structopt(short = "h", help = "Image height in pixels")]
    image_height: usize,

    #[structopt(
        short = "r",
        help = "Specifies after what cycle to print",
        default_value = "100"
    )]
    print_rate: u32,

    #[structopt(short = "c", long, help = "Number of cycles", default_value = "0")]
    cycles: u32,

    #[structopt(short, long, help = "Where to store results", default_value = "./out/")]
    output: String,

    #[structopt(short, long, help = "Run in realtime")]
    live: bool,
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn create_window(width: usize, height: usize) -> Window {
    let mut window = Window::new(
        "Progress",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    return window;
}

fn main() {
    let params: Params = Params::from_args();

    let mut CYCLES: u32 = params.cycles;
    let width: usize = params.image_width;
    let height: usize = params.image_height;

    const POPULATION_SIZE: usize = 300;
    let BEST_NUMBER: u8 = 10;

    let original_bytes = fs::read(params.path).expect("Failed to read file from provided path");

    let mut population: Vec<Vec<u8>> = Vec::with_capacity(POPULATION_SIZE);
    let mut best: Vec<Vec<u8>> = vec![vec![0; original_bytes.len()]; BEST_NUMBER as usize];

    let mut scores: [(usize, usize); POPULATION_SIZE] = [(0, 0); POPULATION_SIZE];

        for i in 0..POPULATION_SIZE {
            population.push(vec![0; original_bytes.len()]);
            scores[i].0 = i;
        }

    let mut rng = rand::thread_rng();

    let mut window = create_window(width, height);

    let mut buffer: Vec<u32> = vec![0; original_bytes.len()];

    if params.live {
        CYCLES = u32::MAX;
    }

    let mut done = false;
    println!("Running...");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if done { 
            window.update_with_buffer(&buffer, width, height).unwrap();
            continue; 
        }
        for i in 0..CYCLES {

            if !window.is_open() { break; }

            cross(&mut population, &best, &mut rng);
            mutate(&mut population, height, width, &mut rng);
            score(&population, &original_bytes, &mut scores);
            note_best_speciment(&population, &mut scores, &mut best);

            let mut counter = 0;
            for byte in &best[0] {
                buffer[counter] = from_u8_rgb(*byte, *byte, *byte);
                counter+=1;
            }
                window.update_with_buffer(&buffer, width, height).unwrap();
            }
        done = true;
        println!("Done!");
        }

        save_best(&best[0]);

   }

fn save_best(best: &Vec<u8>) {
    fs::write("best.raw", best).expect("Failed to write best image to file");
}

fn note_best_speciment<const LENGTH: usize> (
    population: &Vec<Vec<u8>>,
    scores: &mut [(usize, usize); LENGTH],
    best: &mut Vec<Vec<u8>>) {
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

fn cross(population: &mut Vec<Vec<u8>>, best: &Vec<Vec<u8>>, rng: &mut ThreadRng) {
    for i in 0..population.len() {
        // let random_speciment = population.choose_mut(&mut rng).unwrap();
        let random_speciment = &mut population[i];
        let random_best_speciment = best.choose(rng).unwrap();

        for i in 0..random_speciment.len() - 1 {
            random_speciment[i] = random_best_speciment[i];
        }
    }
}

fn mutate(population: &mut Vec<Vec<u8>>, height: usize, width: usize, rng: &mut ThreadRng) {
    for speciment in population {
        let end_x = rng.gen_range(1..width - 1);
        let end_y = rng.gen_range(1..height - 1);

        let start_x = rng.gen_range(1..width - end_x);
        let start_y = rng.gen_range(1..height - end_y);

        let color: u8 = rng.gen_range(0..255);

        for x in end_y..end_y + start_y {
            for y in end_x..end_x + start_x {
                let index = (x * width + y) as usize;
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
