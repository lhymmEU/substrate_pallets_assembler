use generator::{GenAlgoType, Generator};

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let _my_gen = Generator::new().use_algo(GenAlgoType::Default, None, None).unwrap().generate(3);
}
