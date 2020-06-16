fn main() {
    // let mut lalrpop = lalrpop::Configuration::new();
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .emit_rerun_directives(true)
        .emit_report(true)
        .process()
        .unwrap();
}
