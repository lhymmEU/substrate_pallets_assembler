use std::fs;

// store seed to seed_loc with seed_name
pub fn store(seed: &str, seed_loc: &str, seed_name: &str) {
    fs::write(seed_loc.to_owned() + seed_name, seed).expect("Unable to write seeds");
}
// count the number of files within a given directory
pub fn get_file_counts(dir: &str) -> usize {
    let path = fs::read_dir(dir).unwrap();
    path.count()
}
// read a file's content into a String
pub fn read_file_to_string(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
// delete the whole directory and re-create it again
pub fn delete_all(dir: &str) -> Result<(), std::io::Error> {
    fs::remove_dir_all(dir).unwrap();
    fs::create_dir(dir)
}
// clear the generated test files
pub fn clear_test(dir: &str) -> Result<(), std::io::Error> {
    delete_all(dir)
}