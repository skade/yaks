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
    let remove = SubCommand::with_name("remove")
                  .about("remove a yak from the pen")
                  .arg(Arg::with_name("NAME")
                          .help("The name of the sad yak")
                          .required(true)
                          .index(1)
                       );
    let build = SubCommand::with_name("build")
                  .about("build all yaks");
    let publish = SubCommand::with_name("publish")
                  .about("send the yaks off to graze");
    let matches = App::new("yaks")
                             .version("1.0")
                             .author("Florian Gilcher <flo@andersground.net>")
                             .about("manages yaks to shave")
                             .subcommand(list)
                             .subcommand(shave)
                             .subcommand(build)
                             .subcommand(remove)
                             .subcommand(publish)
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

    if let Some(_) = matches.subcommand_matches("build") {
        yaks::build();
    }

    if let Some(_) = matches.subcommand_matches("remove") {
        let name = matches.value_of("NAME").unwrap();

        yaks::remove(name);
    }

    if let Some(_) = matches.subcommand_matches("publish") {
        yaks::publish();
    }
}


