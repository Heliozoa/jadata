use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Generates the kanjifile.
    Kanjifile {
        /// The path to the input KANJIDIC2 file,
        #[arg(short = 'd', long)]
        kanjidic: PathBuf,
        /// The path to the input KRADFILE.
        #[arg(short, long)]
        kradfile: PathBuf,
        /// The path to the kanjifile_skeleton.json file.
        #[arg(short, long)]
        skeleton: PathBuf,
        /// The path to the output kanjifile.
        #[arg(short, long)]
        output: PathBuf,
        /// The format of the output file.
        #[arg(short = 't', long)]
        format: Format,
    },
    /// Generates the wordfile.
    Wordfile {
        /// The path to the input JMdict file.
        #[arg(short, long)]
        jmdict: PathBuf,
        /// The path to the input JMdict furigana file.
        #[arg(short, long)]
        furigana: PathBuf,
        /// The path to the wordfile_skeleton.json file.
        #[arg(short, long)]
        skeleton: PathBuf,
        /// The path to the output wordfile.
        #[arg(short, long)]
        output: PathBuf,
        /// The format of the output file.
        #[arg(short = 't', long)]
        format: Format,
    },
    /// Generates the kanjifile skeleton.
    KanjifileSkeleton {
        /// If set, will generate a fresh skeleton instead of updating an existing one.
        #[arg(short, long, default_value_t = false)]
        clean: bool,
        /// The path to the input KANJIDIC2 file,
        #[arg(short = 'd', long)]
        kanjidic: PathBuf,
        /// The path to the input JMdict file.
        // JMdict contains kanji that are unfortunately not in the KANJIDIC2,
        // so to make sure we don't miss any we go through all the written forms
        // of the JMdict to check for missing kanji
        #[arg(short, long)]
        jmdict: PathBuf,
        /// The path to the output kanjifile skeleton.
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Generates the wordfile skeleton.
    WordfileSkeleton {
        /// If set, will generate a fresh skeleton instead of updating an existing one.
        #[arg(short, long, default_value_t = false)]
        clean: bool,
        /// The path to the input JMdict file.
        #[arg(short, long)]
        jmdict: PathBuf,
        /// The path to the output wordfile skeleton.
        #[arg(short, long)]
        output: PathBuf,
    },
}

/// Alternative formats for the resulting file.
#[derive(Clone, Copy, ValueEnum)]
pub enum Format {
    /// A verbose human-readable and -writable format.
    Json,
    /// A concise binary format. See https://crates.io/crates/postcard.
    Postcard,
}
