/*
    The generator should works in following steps:
        1. it takes in a user defined flag to determine either:
            a) generate test cases from a rule or
            b) use user provided test cases
        2. stamp the test cases using a random order
        3. store the stamped test cases into a database
*/

use generator::{GenAlgoType, Generator};

#[test]
fn generator_template_fluent_api_works() {
    let num = 10;
    let my_gen = Generator::new()
        .use_algo(GenAlgoType::Default, None, None)
        .unwrap()
        .generate(num);
    let seed_location = my_gen.unwrap().initial_seeds_loc.0;

    assert_eq!(num, keeper::get_file_counts(&seed_location) as u32);
    keeper::clear_test(&seed_location).unwrap();
}

#[test]
fn generator_report_error_when_location_is_missing() {
    let my_gen = Generator::new()
        .use_algo(GenAlgoType::Off, None, None);
    match my_gen {
        Ok(_) => { assert_eq!(1, 0) },
        Err(e) => {
            assert_eq!(1, 1)
        },
    }
}
