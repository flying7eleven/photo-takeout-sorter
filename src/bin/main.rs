use chrono::Local;
use clap::{crate_authors, crate_description, crate_name, crate_version, load_yaml, App};
use log::{debug, error, LevelFilter};
use photo_takeout_sorter::{AlbumMetaDataInformation, PhotoMetaInformation};
use std::fs::{read_dir, DirEntry};
use std::io;
use std::path::Path;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                match entry.path().extension() {
                    Some(file_extension) => {
                        if file_extension.eq("json") {
                            cb(&entry);
                        }
                    }
                    None => {}
                }
            }
        }
    }
    Ok(())
}

fn handle_file(dir_entry: &DirEntry) {
    if dir_entry
        .file_name()
        .to_str()
        .unwrap()
        .to_owned()
        .starts_with("Metadat")
    {
        match AlbumMetaDataInformation::from_file(dir_entry.path().to_str().unwrap()) {
            Ok(_) => debug!(
                "{} successfully parsed (meta)",
                dir_entry.path().to_str().unwrap()
            ),
            Err(_) => error!(
                "{} could not be parsed (meta)",
                dir_entry.path().to_str().unwrap()
            ),
        }
    } else {
        match PhotoMetaInformation::from_file(dir_entry.path().to_str().unwrap()) {
            Ok(_) => debug!("{} successfully parsed", dir_entry.path().to_str().unwrap()),
            Err(_) => error!("{} could not be parsed", dir_entry.path().to_str().unwrap()),
        }
    }
}

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
    if matches.occurrences_of("input_folder") != 1 {
        println!("{}", matches.usage());
        return;
    }

    //
    setup_logger(verbosity);

    //
    visit_dirs(
        Path::new(matches.value_of("input_folder").unwrap()),
        &handle_file,
    );
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
