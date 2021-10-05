use rand::prelude::*;
use std::{
    collections::HashSet,
    fs::{self, File},
    io::Error,
};

fn main() {
    const STEPS: u32 = 1000;
    const POPULATION_SIZE: usize = 10;
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 144;

    let original_bytes = fs::read("./smol_papcio.raw").expect("Failed to read bytes");

    let mut population: [Vec<u8>; POPULATION_SIZE] = Default::default();
    let mut scores: [f32; POPULATION_SIZE] = [1.0; POPULATION_SIZE];
    let mut changed: HashSet<u32> = HashSet::with_capacity(POPULATION_SIZE);

    for i in 0..POPULATION_SIZE {
        population[i] = vec![255; original_bytes.len()];
    }

    for i in 0..STEPS {
        println!("Step {}", i + 1);

        cross(&mut population, &mut changed);
        mutate(&mut population, HEIGHT, WIDTH, &mut changed);
        score(&population, &mut scores);

        changed.clear();
    }
}

fn cross<const LENGTH: usize>(population: &mut [Vec<u8>; LENGTH], changed: &mut HashSet<u32>) {}

fn mutate<const LENGTH: usize>(
    population: &mut [Vec<u8>; LENGTH],
    HEIGHT: u32,
    WIDTH: u32,
    changed: &mut HashSet<u32>,
) {
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

fn score<const LENGTH: usize>(population: &[Vec<u8>; LENGTH], scores: &mut [f32; LENGTH]) {}
