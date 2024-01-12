//! Creates the kanjifile and wordfile files.

use clap::Parser;
use jadata_cli::cli::{Cli, Command};

fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Command::Kanjifile {
            kanjidic,
            kradfile,
            skeleton,
            output,
            format,
        } => {
            jadata_cli::create_kanjifile(&kanjidic, &kradfile, &skeleton, &output, format)?;
        }
        Command::Wordfile {
            jmdict,
            furigana,
            skeleton,
            output,
            format,
        } => {
            jadata_cli::create_wordfile(&jmdict, &furigana, &skeleton, &output, format)?;
        }
        Command::KanjifileSkeleton {
            clean,
            kanjidic,
            jmdict,
            output,
        } => {
            if !clean {
                jadata_cli::update_kanjifile_skeleton(&kanjidic, &jmdict, &output)?;
            } else {
                jadata_cli::create_kanjifile_skeleton(&kanjidic, &jmdict, &output)?;
            }
        }
        Command::WordfileSkeleton {
            clean,
            jmdict,
            output,
        } => {
            if !clean {
                jadata_cli::update_wordfile_skeleton(&jmdict, &output)?;
            } else {
                jadata_cli::create_wordfile_skeleton(&jmdict, &output)?;
            }
        }
    }

    Ok(())
}
