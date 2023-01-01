#[derive(Default)]
pub struct Generator {
    // the algorithm used to generate test cases
    algo: GenAlgo,
    // 
    users_seeds_loc: Option<String>,
}
struct GenAlgo(Box<dyn Fn() -> String>);

impl Default for GenAlgo {
    fn default() -> Self {
        GenAlgo(Box::new(|| { "Empty Generation Algorithm".to_string() }))
    }
}

pub enum GenAlgoType {
    Empty,
    Default,
    Customized,
}

impl Generator {
    // build a generator type
    fn build() {

    }
    // generate test cases and store them in a database
    fn run() {

    }
    // generate test cases using customized algorithms
    fn generate(&self, num: u32) -> Vec<String> {
        let mut result = Vec::new();
        // generate #"num" test cases
        for _i in 1..=num {
            result.push((self.algo.0)());
        }

        result
    }

    fn use_algo(mut self, algo_type: GenAlgoType, location: Option<String>, algo: Option<GenAlgo>) -> Generator {
        match algo_type {
            // this type implies that user will provide a location to existing seeds
            GenAlgoType::Empty => {
                self.users_seeds_loc = location;
                self
            },
            // this type implies the user wants to use our default generation algorithms
            GenAlgoType::Default => {
                self.algo = default_gen_algo();
                self
            },
            GenAlgoType::Customized => {
                if let Some(algo) = algo {
                    self.algo = algo;
                }
                self
            }
        }
    }
}

pub fn new() -> Generator {
    Generator::default()
}

// The default generation algorithm
fn default_gen_algo() -> GenAlgo {
    GenAlgo(Box::new( || { 
        "Default Generation Algorithm".to_string()
    }))
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_can_use_customized_default_trait() {
        let g = new();

        assert_eq!(g.generate(1).pop(), Some("Empty Generation Algorithm".to_string()))

    }
    #[test]
    fn use_algo_works_for_different_gen_algo_types() {
        let mut g = new();
        let mut loc = "".to_string();
        let mut customized = "".to_string();
        let mut empty = "".to_string();
        // check if generator can store user provided seeds' location
        g = g.use_algo(
            GenAlgoType::Empty, 
            Some("my seeds location".to_string()), 
            None
        );
        if let Some(s) = &g.users_seeds_loc {
            loc = s.to_string();
        }
        assert_eq!(
            loc,
            "my seeds location".to_string()
        );
        // check if generator can use dafult generation algorithm
        g = g.use_algo(
            GenAlgoType::Default, 
            None, 
            None
        );
        if let Some(s) = &g.generate(1).pop() {
            empty = s.to_string();
        }
        assert_ne!(
            empty,
            "Empty Generation Algorithm".to_string()
        );
        // check if generator can use customized generation algorithm
        g = g.use_algo(
            GenAlgoType::Customized, 
            None, 
            Some(GenAlgo(Box::new(|| { "Customized Generation Algorithm".to_string() })))
        );
        if let Some(s) = &g.generate(1).pop() {
            customized = s.to_string();
        }
        assert_eq!(
            customized,
            "Customized Generation Algorithm".to_string()
        );
    }
    #[test]
    fn use_algo_warns_illegal_gen_algo_types() {
        todo!()
    }
    #[test]
    fn customizable_algorithm_works_for_single_iteration() {
        let mut g = new();

        let algo = GenAlgo(Box::new( || {
            "Hello I'm customized algorithm!".to_string()
        }));

        g = g.use_algo(GenAlgoType::Customized, None, Some(algo));

        assert_eq!(g.generate(1).pop(), Some("Hello I'm customized algorithm!".to_string()))
    }
    #[test]
    fn customizable_algorithm_works_for_multi_iteration() {
        let mut g = new();
        
        let algo = GenAlgo(Box::new( || {
            "Hello I'm customized algorithm!".to_string()
        }));

        g = g.use_algo(GenAlgoType::Customized, None, Some(algo));

        for i in g.generate(10) {
            assert_eq!(i, "Hello I'm customized algorithm!".to_string())
        }
        
    }
    #[test]
    fn generator_can_store_seeds_to_database() {
        todo!()
    }
}
