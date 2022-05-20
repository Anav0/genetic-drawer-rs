use std::cmp::Ordering;

use crate::genetic::Speciment;

pub trait Evaluator<S: Speciment + Clone> {
    fn get_scores_mut(&mut self) -> &mut Vec<(usize, f32)>;
    fn evaluate(&mut self, population: &mut Vec<S>) {
        let scores = self.get_scores_mut();
        for i in 0..population.len() - 1 {
            scores[i].0 = i;
            scores[i].1 = population[i].score();
        }
    }
    fn extract_best(&mut self, best: &mut Vec<S>, population: &Vec<S>) {
        let scores = self.get_scores_mut();
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
}

pub struct DefaultEvaluator {
    scores: Vec<(usize, f32)>,
}

impl<S: Speciment + Clone> Evaluator<S> for DefaultEvaluator {
    fn get_scores_mut(&mut self) -> &mut Vec<(usize, f32)> {
        &mut self.scores
    }
}
