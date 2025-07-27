use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::io::{self,Read};
#[derive(Parser)]
struct Options {
    #[clap(default_value = "meow!")]
    // What does the cat say
    message: String,
    #[clap(short = 'd', long = "dead")]
    // Make the cat appear dead
    dead: bool,
    #[clap(short = 'f', long = "file")]
    /// load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,

    #[clap(short='i', long ="stdin")]
    ///read the message from stdin instead of argument
    stdin:bool,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();
    if options.stdin{
        io::stdin().read_to_string(&mut message)?;
    }else{
    message=options.message;
    };
    let eye = if options.dead { "-" } else { "0" };

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path).with_context(|| {
                format!("Failed to read cat picture from file: {}", path.display())
            })?;
            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", &cat_picture);
        }
        None => {
            let eye = if options.dead { "-" } else { "0" };

            println!("( {message} )");
            println!(" \\");
            println!("  \\");
            println!("   /\\_/\\");
            println!("  ( {eye} {eye} )", eye = eye.red().bold());
            println!("  =( | )= ");
        }
    }
    Ok(())
}
