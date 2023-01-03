// TODO: add tracing to trace the execution of generator, for future metering & web UI to use
// TODO: add error handling for future web UI to use

#[derive(Default)]
pub struct Generator {
    // the algorithm used to generate test cases
    pub algo: GenAlgo,
    // record the generation type used
    pub algo_type: GenAlgoType,
    // to store user provided seeds' location, if any
    pub users_seeds_loc: SeedLoc,
    // the location of the initial seeds
    pub initial_seeds_loc: SeedLoc,
}

impl Generator {
    // new will return a default Generator
    pub fn new() -> Generator {
        Generator::default()
    }
    // set the algorithm to use
    pub fn use_algo(
        mut self,
        algo_type: GenAlgoType,
        location: Option<SeedLoc>,
        algo: Option<GenAlgo>,
    ) -> Result<Generator, String> {
        match algo_type {
            // this type implies that user will provide a location to existing seeds
            GenAlgoType::Off => {
                if let Some(loc) = location {
                    self.users_seeds_loc = loc;
                } else {
                    // TODO: need to change this
                    panic!("A location of initial seeds must be provided if GenAlgo is Off!")
                }
                self.algo_type = GenAlgoType::Off;
                Ok(self)
            }
            // this type means the user wants to use our default generation algorithms
            GenAlgoType::Default => {
                self.algo = default_gen_algo();
                self.algo_type = GenAlgoType::Default;
                Ok(self)
            }
            // this type means the user want to provide their own algo to use
            GenAlgoType::Customized => {
                if let Some(algo) = algo {
                    self.algo = algo;
                }
                self.algo_type = GenAlgoType::Customized;
                Ok(self)
            }
        }
    }
    // generate test cases using customized algorithms,
    // then store them into database.
    pub fn generate(mut self, num: u32) -> Result<Generator, String> {
        // if the seeds are provided by user
        // store the user provided location as initial seeds location
        match self.algo_type {
            GenAlgoType::Off => {
                // if user provided some initial seeds,
                // just change the location to it, no need to copy them over
                self.initial_seeds_loc = SeedLoc(self.users_seeds_loc.0.to_owned());
                Ok(self)
            }
            GenAlgoType::Customized | GenAlgoType::Default => {
                for i in 1..=num {
                    keeper::store(
                        (self.algo.0)().as_str(),
                        self.initial_seeds_loc.0.as_str(),
                        // TODO: consider add algorithm's name to the seed name
                        &format!("/initial_seed_{}", i),
                    );
                }
                Ok(self)
            }
        }
    }
}

// gen algo is stored in the Generator as a boxed closure
// TODO: need to dig into Fn(), FnMut(), FnOnce(), and fn.
pub struct GenAlgo(Box<dyn Fn() -> String>);
impl Default for GenAlgo {
    // notice: default() here implements an empty algo for initialization,
    // this is not to be confused with the default algorithms provided by this crate
    fn default() -> Self {
        GenAlgo(Box::new(|| "Empty Generation Algorithm".to_string()))
    }
}

// the location of seeds for both initial and mutated ones
pub struct SeedLoc(pub String);
impl Default for SeedLoc {
    fn default() -> Self {
        SeedLoc("../seeds/initial".to_string())
    }
}

// acceptable generation algorithm types
pub enum GenAlgoType {
    // do not use gen algo, use provided seeds instead
    Off,
    // use default generation algorithms,
    // TODO: need to add more options to default, ideally one for each gen algo.
    Default,
    // use user customized algorithms
    Customized,
}

impl Default for GenAlgoType {
    fn default() -> Self {
        GenAlgoType::Default
    }
}

// The default generation algorithm
fn default_gen_algo() -> GenAlgo {
    GenAlgo(Box::new(|| "Default Generation Algorithm".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    // we need to make these tests serial because
    // they manipulate file systems
    use serial_test::serial;

    #[test]
    #[serial]
    fn generator_can_use_customized_default_trait() {
        // initialize generator
        let mut g = Generator::new();
        // prepare tests
        g = g.generate(1).unwrap();
        let file_content = keeper::read_file_to_string(&format!(
            "{}{}",
            &g.initial_seeds_loc.0, "/initial_seed_1"
        ))
        .unwrap();
        // make assertions
        assert_eq!(file_content, "Empty Generation Algorithm".to_string());
        // reset file system state
        keeper::clear_test(&g.initial_seeds_loc.0).unwrap();
    }

    #[test]
    #[serial]
    fn use_algo_works_for_different_gen_algo_types() {
        let mut g = Generator::new();
        // check if generator can store user provided seeds' location
        g = g
            .use_algo(
                GenAlgoType::Off,
                Some(SeedLoc("my seeds location".to_string())),
                None,
            )
            .unwrap();

        assert_eq!(g.users_seeds_loc.0, "my seeds location".to_string());

        // check if generator can use default generation algorithm
        g = g
            .use_algo(GenAlgoType::Default, None, None)
            .unwrap()
            .generate(1)
            .unwrap();
        let file_path = g.initial_seeds_loc.0.to_owned() + "/initial_seed_1";
        let empty = keeper::read_file_to_string(&file_path).unwrap();

        assert_ne!(empty, "Empty Generation Algorithm".to_string());
        keeper::clear_test(&g.initial_seeds_loc.0).unwrap();

        // check if generator can use customized generation algorithm
        g = g
            .use_algo(
                GenAlgoType::Customized,
                None,
                Some(GenAlgo(Box::new(|| {
                    "Customized Generation Algorithm".to_string()
                }))),
            )
            .unwrap()
            .generate(1)
            .unwrap();
        let customized = keeper::read_file_to_string(&file_path).unwrap();

        assert_eq!(customized, "Customized Generation Algorithm".to_string());
        keeper::clear_test(&g.initial_seeds_loc.0).unwrap();
    }

    #[test]
    #[serial]
    fn customizable_algorithm_works_for_single_iteration() {
        let mut g = Generator::new();
        let algo = GenAlgo(Box::new(|| "Hello I'm customized algorithm!".to_string()));

        g = g
            .use_algo(GenAlgoType::Customized, None, Some(algo))
            .unwrap()
            .generate(1)
            .unwrap();
        let content = keeper::read_file_to_string(&format!(
            "{}{}",
            &g.initial_seeds_loc.0, "/initial_seed_1"
        ))
        .unwrap();

        assert_eq!(content, "Hello I'm customized algorithm!".to_string());
        keeper::clear_test(&g.initial_seeds_loc.0).unwrap();
    }

    #[test]
    #[serial]
    fn customizable_algorithm_works_for_multi_iteration() {
        let mut g = Generator::new();
        let num = 10;
        let algo = GenAlgo(Box::new(|| "Hello I'm customized algorithm!".to_string()));

        g = g
            .use_algo(GenAlgoType::Customized, None, Some(algo))
            .unwrap()
            .generate(num)
            .unwrap();
        let file_counts = keeper::get_file_counts(&g.initial_seeds_loc.0);

        assert_eq!(num, file_counts as u32);
        keeper::clear_test(&g.initial_seeds_loc.0).unwrap();
    }

    #[test]
    #[serial]
    fn generator_can_read_user_provided_seeds() {
        let mut g = Generator::new();
        let test_seeds_num = 3;

        g = g
            .use_algo(
                GenAlgoType::Off,
                Some(SeedLoc("../seeds/test_seeds".to_string())),
                None,
            )
            .unwrap()
            .generate(0)
            .unwrap();
        let file_counts = keeper::get_file_counts(&g.initial_seeds_loc.0);

        assert_eq!(test_seeds_num, file_counts as u32);
        // we do not delete the test seeds after the test
    }

    #[test]
    #[serial]
    fn generator_can_store_seeds_to_database() {
        let mut g = Generator::new();
        let num = 3;

        g = g.generate(num).unwrap();
        let result = keeper::get_file_counts(&g.initial_seeds_loc.0);

        assert_eq!(num, result as u32);
        keeper::clear_test(&g.initial_seeds_loc.0).unwrap();
    }
}
