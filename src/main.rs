extern crate clap;
extern crate yaks;

use clap::{Arg, App, SubCommand};

fn main() {
    let yaks_dir = std::env::var("YAKS_DIR").ok().unwrap_or(".".into());

    assert!(std::env::set_current_dir(yaks_dir).is_ok());

    let list = SubCommand::with_name("list")
                  .about("lists yaks");
    let shave = SubCommand::with_name("shave")
                  .about("shave a yak")
                  .arg(Arg::with_name("NAME")
                          .help("The name of the yak to shave")
                          .required(true)
                          .index(1)
                       );
    let matches = App::new("yaks")
                             .version("1.0")
                             .author("flo@andersground.net")
                             .about("manages yaks to shave")
                             .subcommand(list)
                             .subcommand(shave)
                             .get_matches();

    if let Some(_) = matches.subcommand_matches("list") {
        for yak in yaks::list() {
            println!("{}", yak.name);
        }
    }

    if let Some(matches) = matches.subcommand_matches("shave") {
        let name = matches.value_of("NAME").unwrap();

        yaks::edit(name);
    }

}


