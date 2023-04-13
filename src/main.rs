mod install;
mod util;


use clap::{Parser, Arg};
use color_eyre::Result;


#[derive(Parser, Debug, Clone)]
#[command(author=env!("CARGO_PKG_AUTHORS"), version=env!("CARGO_PKG_VERSION"), about=env!("CARGO_PKG_DESCRIPTION"), long_about=format!("{}\n{}", env!("CARGO_PKG_DESCRIPTION"), env!("CARGO_PKG_AUTHORS").replace(':', "\n")),)]
pub struct Args {

    /// update Selected or all mclang parts
    /// Usage:
    ///     mclang-up --update [component]
    #[arg(long, short)]
    update: bool,

    /// update Selected or all mclang parts
    /// Usage:
    ///     mclang-up --update [component]@[version]
    #[arg(long, short)]
    install: bool,
    
    /// Print more info
    #[arg(long, short)]
    verbose: bool,

    // The component to update
    #[arg(long, short, default_value_t=String::from("all"))]
    component: String

}

fn main() -> Result<()>{
    let args = Args::parse();


    if args.install {
        if let Err(_) = install::install(&args) {
            error!("Instalation failed");
            return Ok(());
        }
    } else if  args.update {
        if let Err(_) = install::update(&args) {
            error!("Update failed");
            return Ok(());
        }
    }
    Ok(())
}
