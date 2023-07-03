use clap::{App, Arg};
use std::error::Error;

pub fn parse_cli_args() -> Result<(String, bool), Box<dyn Error>> {
    let matches = App::new("My CLI App")
        .version("1.0")
        .about("Fetches npm package versions")
        .arg(Arg::with_name("package")
            .short("p".parse().unwrap())
            .long("package")
            .value_name("PACKAGE")
            .takes_value(true)
            .required(true)
        )
        .arg(
            Arg::with_name("private")
                .short("r".parse().unwrap())
                .long("private")
                .help("Specifies that the package is located in a private Artifactory repository")
                .takes_value(false),
        )
        .get_matches();


    let package_name = matches.value_of("package").ok_or("No package name provided")?;
    let private = matches.is_present("private");
    return Ok((package_name.to_string(), private));
}
