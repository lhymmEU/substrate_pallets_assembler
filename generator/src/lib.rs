#[derive(Default)]
pub struct Generator {
    // the algorithm used to generate test cases
    algo: GenAlgo,
    // to store user provided seeds' location, if any
    users_seeds_loc: Option<String>,
    // the location of the initial seeds
    initial_seeds_loc: SeedLoc,
}
struct GenAlgo(Box<dyn Fn() -> String>);
impl Default for GenAlgo {
    fn default() -> Self {
        GenAlgo(Box::new(|| "Empty Generation Algorithm".to_string()))
    }
}

struct SeedLoc(String);
impl Default for SeedLoc {
    fn default() -> Self {
        SeedLoc("../seeds/initial".to_string())
    }
}

pub enum GenAlgoType {
    Empty,
    Default,
    Customized,
}

impl Generator {
    // generate test cases using customized algorithms,
    // then store them into database.
    fn generate(mut self, num: u32) -> Result<Generator, String> {
        // if the seeds are provided by user
        // store the user provided location as initial seeds location
        if num == 0 {
            if let Some(seed_loc) = &self.users_seeds_loc {
                self.initial_seeds_loc = SeedLoc(seed_loc.to_string());
            }
            Ok(self)
        } else {
            // generate and store #"num" test cases
            for i in 1..=num {
                keeper::store(
                    (self.algo.0)().as_str(),
                    self.initial_seeds_loc.0.as_str(),
                    &format!("/initial_seed_{}", i),
                );
            }
            Ok(self)
        }
    }

    fn use_algo(
        mut self,
        algo_type: GenAlgoType,
        location: Option<String>,
        algo: Option<GenAlgo>,
    ) -> Generator {
        match algo_type {
            // this type implies that user will provide a location to existing seeds
            GenAlgoType::Empty => {
                self.users_seeds_loc = location;
                self
            }
            // this type implies the user wants to use our default generation algorithms
            GenAlgoType::Default => {
                self.algo = default_gen_algo();
                self
            }
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
    GenAlgo(Box::new(|| "Default Generation Algorithm".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn clear_test(dir: &str) -> Result<(), std::io::Error> {
        keeper::delete_all(dir)
    }

    #[test]
    #[serial]
    fn generator_can_use_customized_default_trait() {
        let mut g = new();

        g = g.generate(1).unwrap();
        let file_content = keeper::read_file_to_string(&format!("{}{}", &g.initial_seeds_loc.0, "/initial_seed_1")).unwrap();
        assert_eq!(
            file_content,
            "Empty Generation Algorithm".to_string()
        );

        clear_test(&g.initial_seeds_loc.0).unwrap();
    }
    #[test]
    #[serial]
    fn use_algo_works_for_different_gen_algo_types() {
        let mut g = new();
        let mut loc = "".to_string();
        let mut customized = "".to_string();
        let mut empty = "".to_string();
        // check if generator can store user provided seeds' location
        g = g.use_algo(
            GenAlgoType::Empty,
            Some("my seeds location".to_string()),
            None,
        );
        if let Some(s) = &g.users_seeds_loc {
            loc = s.to_string();
        }
        assert_eq!(loc, "my seeds location".to_string());
        // check if generator can use default generation algorithm
        g = g
            .use_algo(GenAlgoType::Default, None, None)
            .generate(1)
            .unwrap();
        let file_path = g.initial_seeds_loc.0.to_owned() + "/initial_seed_1";
        empty = keeper::read_file_to_string(&file_path).unwrap();
        assert_ne!(empty, "Empty Generation Algorithm".to_string());
        clear_test(&g.initial_seeds_loc.0).unwrap();
        // check if generator can use customized generation algorithm
        g = g
            .use_algo(
            GenAlgoType::Customized,
            None,
            Some(GenAlgo(Box::new(|| {
                "Customized Generation Algorithm".to_string()
            }))),
            )
            .generate(1)
            .unwrap();
        customized = keeper::read_file_to_string(&file_path).unwrap();
        assert_eq!(customized, "Customized Generation Algorithm".to_string());
        clear_test(&g.initial_seeds_loc.0).unwrap();
    }
    #[test]
    #[serial]
    fn use_algo_warns_illegal_gen_algo_types() {
        todo!()
    }
    #[test]
    #[serial]
    fn customizable_algorithm_works_for_single_iteration() {
        let mut g = new();

        let algo = GenAlgo(Box::new(|| "Hello I'm customized algorithm!".to_string()));

        g = g
            .use_algo(GenAlgoType::Customized, None, Some(algo))
            .generate(1)
            .unwrap();
        let content = keeper::read_file_to_string(&format!("{}{}", &g.initial_seeds_loc.0, "/initial_seed_1")).unwrap();
        assert_eq!(
            content,
            "Hello I'm customized algorithm!".to_string()
        );
        clear_test(&g.initial_seeds_loc.0).unwrap();
    }
    #[test]
    #[serial]
    fn customizable_algorithm_works_for_multi_iteration() {
        let mut g = new();
        let num = 10;

        let algo = GenAlgo(Box::new(|| "Hello I'm customized algorithm!".to_string()));

        g = g
            .use_algo(GenAlgoType::Customized, None, Some(algo))
            .generate(num)
            .unwrap();

        let file_counts = keeper::get_file_counts(&g.initial_seeds_loc.0);

        assert_eq!(num, file_counts as u32);
        clear_test(&g.initial_seeds_loc.0).unwrap();
    }
    #[test]
    #[serial]
    fn generator_can_read_user_provided_seeds() {
        let mut g = new();
        let test_seeds_num = 3;

        g = g
            .use_algo(
                GenAlgoType::Empty,
                Some("../seeds/test_seeds".to_string()),
                None,
            )
            .generate(0)
            .unwrap();

        let file_counts = keeper::get_file_counts(&g.initial_seeds_loc.0);
        assert_eq!(test_seeds_num, file_counts as u32);
        clear_test(&g.initial_seeds_loc.0).unwrap();
    }
    #[test]
    #[serial]
    fn generator_can_store_seeds_to_database() {
        let mut g = new();
        let num = 3;

        g = g.generate(num).unwrap();
        let result = keeper::get_file_counts(&g.initial_seeds_loc.0);

        assert_eq!(num, result as u32);
        clear_test(&g.initial_seeds_loc.0).unwrap();
    }
}
