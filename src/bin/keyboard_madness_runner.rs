use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct KeyboardMadness {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Run(RunArgs),
    Generate(GenerateArgs),
}

/// Run instructions on the keyboard
#[derive(Parser, Debug)]
#[command(name = "run", author, version, about, long_about = None)]
struct RunArgs {
    /// X starting position on the keyboard
    #[arg(short, default_value_t = 4)]
    x_position: usize,

    /// Y starting position on the keyboard
    #[arg(short, default_value_t = 2)]
    y_position: usize,

    /// Instructions to execute
    #[clap(default_value = "R,S,U,L:3,S,D,R:6,S,S,U,S")]
    instructions: String,
}

/// Generate instructions
#[derive(Parser, Debug)]
#[command(name = "generate", author, version, about, long_about = None)]
struct GenerateArgs {
    /// X starting position on the keyboard
    #[arg(short, default_value_t = 4)]
    x_position: usize,

    /// Y starting position on the keyboard
    #[arg(short, default_value_t = 2)]
    y_position: usize,

    /// Input text
    #[clap(default_value = "Hello")]
    text: String,
}

fn main() {
    let args = KeyboardMadness::parse();

    match args.command {
        Command::Run(run_args) => {
            let mut keyboard = keyboard_madness::Keyboard {
                keyboard_layout: keyboard_madness::KEYS,
                position: (run_args.x_position, run_args.y_position),
                selected_keys: &mut vec![],
            };
            keyboard.run(&run_args.instructions);
            println!("{}", keyboard);
        }
        Command::Generate(generate_args) => {
            let mut keyboard = keyboard_madness::Keyboard {
                keyboard_layout: keyboard_madness::KEYS,
                position: (generate_args.x_position, generate_args.y_position),
                selected_keys: &mut vec![],
            };

            println!(
                "{}",
                keyboard.generate_instructions(&generate_args.text.to_ascii_uppercase())
            );
        }
    }
}
