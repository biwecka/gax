// Stats
pub struct Stats {
    generation: usize,
    current_best: Cost,
}

impl ga::stats::Stats<Cost, Context, QueenPos> for Stats {
    fn generation(&self) -> usize {
        self.generation
    }

    fn inc_gen(&mut self) {
        self.generation += 1;
    }

    fn curr_best(&self) -> Cost {
        self.current_best
    }

    fn update(
        &mut self,
        population: &[(QueenPos, Cost)],
        distinct_selections: usize,
    ) {
        // Set current best
        if let Some(best) = population.first() {
            self.current_best = best.1;
        }
    }

    fn log(&self) {}
}