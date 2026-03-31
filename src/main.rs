use anyhow::Result;
use clap::{arg, command, value_parser, Arg, ArgMatches, Command};
use std::env;
use tokio;

pub mod export;
pub mod import;
pub mod init;
pub mod ofn_2_owl;
pub mod owl_2_ofn;
pub mod prefix;
pub mod xml;
extern crate wiring_rs;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = parse_arguments();

    let exit_result = match matches.subcommand() {
        Some(("init", sub_matches)) => {
            init::init::init(sub_matches).await?;
            Ok(())
        }
        Some(("prefix", sub_matches)) => {
            prefix::prefix::prefix(sub_matches).await?;
            Ok(())
        }
        Some(("import", sub_matches)) => {
            import::import::import(sub_matches).await?;
            Ok(())
        }
        Some(("export", sub_matches)) => {
            export::export::export(sub_matches).await?;
            Ok(())
        }
        _ => anyhow::bail!("Unrecognized or missing subcommand."),
    };

    exit_result
}

fn parse_arguments() -> ArgMatches {
    command!()
        .about("This application is a CLI for converting ontologies") // requires `cargo` feature
        .subcommand_required(true)
        .subcommand(
            Command::new("init")
                .about("Initialises LDTab database")
                .arg(
                    Arg::new("input")
                        .index(1) // positional arguments can't have a short/long name
                        .help("The input file to process")
                        .conflicts_with("connection"),
                )
                .arg(
                    arg!(
                        -c --connection <URL>  "Specifies a database connection URL or file"
                    )
                    .required(false)
                    .value_parser(value_parser!(String)),
                ),
        )
        .subcommand(
            Command::new("prefix")
                .about("Configure prefixes for an LDTab database")
                .arg(
                    Arg::new("database")
                        .index(1) // The position of the argument in the command line
                        .required(true)
                        .help("The database"),
                )
                .arg(
                    Arg::new("prefixes")
                        .index(2) // The position of the argument in the command line
                        .required(true)
                        .help("The prefix TSV file"),
                ),
        )
        .subcommand(
            Command::new("import")
                .about("Imports an LDTab database")
                .arg(
                    Arg::new("database")
                        .index(1) // The position of the argument in the command line
                        .required(true)
                        .help("The target database to import into"),
                )
                .arg(
                    Arg::new("ontology")
                        .index(2) // The position of the argument in the command line
                        .required(true)
                        .help("The ontology to be imported"),
                ),
        )
        .subcommand(
            Command::new("export")
                .about("Exports an LDTab database")
                .arg(
                    Arg::new("database")
                        .index(1) // The position of the argument in the command line
                        .required(true)
                        .help("The datapase to export"),
                )
                .arg(
                    Arg::new("output")
                        .index(2) // The position of the argument in the command line
                        .required(true)
                        .help("The target file to export to"),
                ),
        )
        .get_matches()
}
