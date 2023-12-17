//! Creates the `kanjifile.json` and `wordfile.json` files.

mod cli;
mod input;
mod output;

use self::input::{jmdict::JMdict, jmdict_furigana, kanjidic2::Kanjidic2, kradfile::Kradfile};
use crate::output::{kanjifile, kanjifile_skeleton, wordfile, wordfile_skeleton};
use clap::Parser;
use cli::{Cli, Command, Format};
use eyre::{ContextCompat, WrapErr};
use jadata::{kanjifile::Kanjifile, wordfile::Wordfile};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write},
    path::Path,
};

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
            create_kanjifile(&kanjidic, &kradfile, &skeleton, &output, format)?;
        }
        Command::Wordfile {
            jmdict,
            furigana,
            skeleton,
            output,
            format,
        } => {
            create_wordfile(&jmdict, &furigana, &skeleton, &output, format)?;
        }
        Command::KanjifileSkeleton { kanjidic, output } => {
            create_kanjifile_skeleton(&kanjidic, &output)?;
        }
        Command::WordfileSkeleton { jmdict, output } => {
            create_wordfile_skeleton(&jmdict, &output)?;
        }
    }

    Ok(())
}

fn create_kanjifile(
    kanjidic_path: &Path,
    kradfile_path: &Path,
    skeleton_path: &Path,
    output_path: &Path,
    format: Format,
) -> eyre::Result<()> {
    tracing::info!("opening files");
    let kd2 = open(kanjidic_path)?;
    let kf = open(kradfile_path)?;
    let kfs = open(skeleton_path)?;

    tracing::info!("deserializing files");
    let kd2: Kanjidic2 = serde_xml_rs::from_reader(BufReader::new(kd2))?;
    let kf: Kradfile = Kradfile::from(BufReader::new(kf))?;
    let mut kfs: Kanjifile = serde_json::from_reader(BufReader::new(kfs))?;

    tracing::info!("producing kanjifile");
    kanjifile::fill_skeleton(&mut kfs, kd2, kf);

    tracing::info!("writing output");
    let kf = File::create(output_path)?;
    let mut kf = BufWriter::new(kf);
    match format {
        Format::Json => {
            serde_json::to_writer_pretty(kf, &kfs)?;
        }
        Format::Postcard => {
            let serialized = postcard::to_stdvec(&kfs)?;
            kf.write_all(&serialized)?;
        }
    }
    Ok(())
}

fn create_wordfile(
    jmdict: &Path,
    jmdict_furigana: &Path,
    skeleton_path: &Path,
    output: &Path,
    format: Format,
) -> eyre::Result<()> {
    tracing::info!("opening files");
    let jmdict = open(jmdict)?;
    let furigana = open(jmdict_furigana)?;
    let wfs = open(skeleton_path)?;

    tracing::info!("parsing jmdict version");
    let version = parse_jmdict_version(&jmdict)?;

    tracing::info!("deserializing");
    let jmdict = JMdict::deserialize(jmdict)?;
    let furigana: Vec<jmdict_furigana::Furigana> =
        serde_json::from_reader(BufReader::new(furigana))?;
    let mut wfs: Wordfile = serde_json::from_reader(BufReader::new(wfs))?;

    tracing::info!("producing wordfile");
    wordfile::fill_skeleton(&mut wfs, jmdict, version, furigana)?;

    tracing::info!("writing output");
    let wf = File::create(output)?;
    let mut wf = BufWriter::new(wf);
    match format {
        Format::Json => {
            serde_json::to_writer_pretty(wf, &wfs)?;
        }
        Format::Postcard => {
            let serialized = postcard::to_stdvec(&wfs)?;
            wf.write_all(&serialized)?;
        }
    }
    Ok(())
}

fn create_kanjifile_skeleton(kanjidic_path: &Path, output_path: &Path) -> eyre::Result<()> {
    tracing::info!("opening files");
    let kd2 = open(kanjidic_path)?;

    tracing::info!("deserializing files");
    let kd2: Kanjidic2 = serde_xml_rs::from_reader(BufReader::new(kd2))?;

    tracing::info!("producing kanjifile skeleton");
    let skeleton = kanjifile_skeleton::create(kd2)?;

    tracing::info!("writing output");
    let kf = File::create(output_path)?;
    serde_json::to_writer_pretty(BufWriter::new(kf), &skeleton)?;
    Ok(())
}

fn create_wordfile_skeleton(jmdict: &Path, output: &Path) -> eyre::Result<()> {
    tracing::info!("opening files");
    let jmdict = open(jmdict)?;

    tracing::info!("parsing jmdict version");
    let version = parse_jmdict_version(&jmdict)?;

    tracing::info!("deserializing");
    let jmdict = JMdict::deserialize(BufReader::new(jmdict))?;

    tracing::info!("producing wordfile");
    let skeleton = wordfile_skeleton::create(jmdict, version)?;

    tracing::info!("writing output");
    let wf = File::create(output)?;
    serde_json::to_writer_pretty(BufWriter::new(wf), &skeleton)?;
    Ok(())
}

fn open(path: &Path) -> eyre::Result<File> {
    File::open(path).wrap_err_with(|| format!("Failed to open file at '{}'", path.display()))
}

fn parse_jmdict_version(jmdict: &File) -> eyre::Result<String> {
    let lines = BufReader::new(jmdict.try_clone()?).lines();
    let mut jmdict = BufReader::new(jmdict);
    let version = lines
        .take(10) // the rev should be in the first couple lines
        .find_map(|l| match l {
            Ok(l) => {
                let (_, rev) = l.split_once("<!-- Rev ")?;
                Some(Ok(rev.to_string()))
            }
            Err(err) => Some(Err(err)),
        })
        .wrap_err("No revision found in jmdict file")??;
    jmdict.seek(SeekFrom::Start(0))?;
    Ok(version)
}
