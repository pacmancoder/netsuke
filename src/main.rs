extern crate clap;
use clap::{App, Arg, SubCommand};

fn main()
{
    println!("test");

    let app = App::new("netsuke")
            .version("0.1.0")
            .version_short("v")
            .author("Vladislav Nikonov <pacmancoder@gmail.com>")
            .about("Config file management suite")
            .help("Does nothing. It is 0.1.0 dude!")
            .help_short("h")
        .get_matches();
}