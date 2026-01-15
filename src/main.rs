mod program;

fn main() {
    let program = program::Program::new();
    let args: Vec<String> = std::env::args().collect();
    program.run(args);
}
