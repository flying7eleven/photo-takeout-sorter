use chrono::Local;
use clap::{crate_authors, crate_description, crate_name, crate_version, load_yaml, App};
use log::LevelFilter;

fn main() {
    // configure the command line parser
    let configuration_parser_config = load_yaml!("cli.yml");
    let matches = App::from_yaml(configuration_parser_config)
        .author(crate_authors!())
        .version(crate_version!())
        .name(crate_name!())
        .about(crate_description!())
        .get_matches();

    //
    let verbosity = match matches.occurrences_of("v") {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2 | _ => LevelFilter::Trace,
    };

    //
    setup_logger(verbosity);
}

fn setup_logger(verbosity: LevelFilter) {
    let _ = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(verbosity)
        .chain(std::io::stdout())
        .apply();
}
