use clap::Parser;

/// Rust echo
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input text
    #[arg(required(true))]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short('n'))]
    omit_new_line: bool,
}

fn main() {
    let args = Args::parse();

    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_new_line { "" } else { "\n" }
    );
}
