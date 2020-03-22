use ants::{self, DqnStrategy, StartState, TensorflowDqn};
use argparse::{ArgumentParser, Store};
use std::error::Error;
use std::io;
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
    let mut model = "model".to_string();
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("A DQN based ants bot.");
        parser
            .refer(&mut model)
            .add_option(&["-m", "--model"], Store, "Path to model directory");
        parser.parse_args_or_exit();
    }
    let dqn = TensorflowDqn::new(
        model.as_str(),
        "serving_default_input_1",
        "StatefulPartitionedCall",
    )?;

    let mut strategy = DqnStrategy::new(dqn);
    let start_state = StartState::new(&mut strategy);

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();
    ants::run(start_state, &mut input, &mut output);

    Ok(())
}
