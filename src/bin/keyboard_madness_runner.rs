use clap::Parser;

/// Simple program for execute instruction for positional typing on a keyboard
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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

fn main() {
    let args = Args::parse();

    let mut keyboard = keyboard_madness::Keyboard {
        keyboard_layout: keyboard_madness::KEYS,
        position: (args.x_position, args.y_position),
        selected_keys: &mut vec![],
    };

    keyboard.run(args.instructions);
    println!("{}", keyboard);
}
