use server::{routes::hello_from_assemble, start::start_server};
fn main() {
    hello_from_assemble();
    start_server();
}
