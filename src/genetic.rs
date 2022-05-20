pub use crate::evaluator::Evaluator;

pub trait Speciment {
    fn score(&self) -> f32;
}

pub trait Crosser<S: Speciment> {
    fn cross(&self, population: &mut Vec<S>, best: &Vec<S>);
}

pub trait Mutator<S: Speciment> {
    fn mutate(&self, population: &mut Vec<S>);
}

pub struct GeneticAlgorithm<'a, S: Speciment> {
    pub print_rate: u32,
    crosser: &'a mut dyn Crosser<S>,
    mutator: &'a mut dyn Mutator<S>,
    evaluator: &'a mut dyn Evaluator<S>,
}

impl<'a, S: Speciment + Clone> GeneticAlgorithm<'a, S> {
    pub fn new(
        print_rate: u32,
        crosser: &'a mut dyn Crosser<S>,
        mutator: &'a mut dyn Mutator<S>,
        evaluator: &'a mut dyn Evaluator<S>,
    ) -> Self {
        Self {
            print_rate,
            crosser,
            mutator,
            evaluator,
        }
    }

    pub fn evolve(&mut self, population: &mut Vec<S>, cycles: u32) {
        let mut best: Vec<S> = vec![population[0].clone(); 10];

        for i in 0..cycles {
            self.mutator.mutate(population);
            self.crosser.cross(population, &best);
            self.evaluator.evaluate(population);
            self.evaluator.extract_best(&mut best, population);

            if i % self.print_rate == 0 {
                println!("Cycle {}", i);
            }
        }
    }
}
