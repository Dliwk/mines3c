use lalrpop;

fn main() {
    println!("cargo:rerun-if-changed=src/grammar.lalrpop");

    // У меня CLion не умеет обрабатывать lalrpop_mod!(), так что да
    lalrpop::Configuration::new()
        .generate_in_source_tree()
        .process()
        .unwrap();
}
