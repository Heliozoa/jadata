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
        /// The path to the input JMDICT file.
        #[arg(short, long)]
        jmdict: PathBuf,
        /// The path to the input JMDICT furigana file.
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
        #[arg(short, long, default_value_t = true)]
        clean: bool,
        /// The path to the input KANJIDIC2 file,
        #[arg(short = 'd', long)]
        kanjidic: PathBuf,
        /// The path to the output kanjifile skeleton.
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Generates the wordfile skeleton.
    WordfileSkeleton {
        /// If set, will generate a fresh skeleton instead of updating an existing one.
        #[arg(short, long, default_value_t = true)]
        clean: bool,
        /// The path to the input JMDICT file.
        #[arg(short, long)]
        jmdict: PathBuf,
        /// The path to the output wordfile skeleton.
        #[arg(short, long)]
        output: PathBuf,
    },
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Format {
    Json,
    Postcard,
}
