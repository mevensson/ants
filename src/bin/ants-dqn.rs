use ants::{self, DqnStrategy, StartState, TensorflowDqn};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process::exit;
use std::result::Result;

fn main() {
    exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            println!("{}", e);
            1
        }
    })
}

fn run() -> Result<(), Box<dyn Error>> {
    let filename = "model.pb";
    let mut model_data = Vec::new();
    File::open(filename)?.read_to_end(&mut model_data)?;
    let dqn = TensorflowDqn::new(&model_data, "flatten_input", "dense_1/Softmax")?;

    let mut strategy = DqnStrategy::new(dqn);
    let start_state = StartState::new(&mut strategy);

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();
    ants::run(start_state, &mut input, &mut output);

    Ok(())
}
