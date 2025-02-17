use ansi_term::{Colour, Style};

fn main() {
    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );
    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );
}
