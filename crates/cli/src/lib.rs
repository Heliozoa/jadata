pub mod cli;
pub mod input;
pub mod output;

use self::{
    cli::Format,
    input::{jmdict::JMdict, jmdict_furigana, kanjidic2::Kanjidic2, kradfile::Kradfile},
    output::{kanjifile, kanjifile_skeleton, wordfile, wordfile_skeleton},
};
use eyre::{ContextCompat, WrapErr};
use jadata::{kanjifile::Kanjifile, wordfile::Wordfile};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write},
    path::Path,
};

// todo: share code between create/update

pub fn create_kanjifile(
    version: String,
    kanjidic: &Path,
    kradfile: &Path,
    skeleton: &Path,
    output: &Path,
    format: Format,
) -> eyre::Result<()> {
    tracing::info!("opening files");
    let kd2 = open(kanjidic)?;
    let kf = open(kradfile)?;
    let kfs = open(skeleton)?;

    tracing::info!("deserializing files");
    let kd2: Kanjidic2 = serde_xml_rs::from_reader(BufReader::new(kd2))?;
    let kf: Kradfile = Kradfile::from(BufReader::new(kf))?;
    let mut kfs: Kanjifile = serde_json::from_reader(BufReader::new(kfs))?;

    tracing::info!("producing kanjifile");
    kanjifile::fill_skeleton(&mut kfs, version, kd2, kf);

    tracing::info!("writing output");
    let kf = File::create(output)?;
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

pub fn create_wordfile(
    version: String,
    jmdict: &Path,
    jmdict_furigana: &Path,
    skeleton: &Path,
    output: &Path,
    format: Format,
) -> eyre::Result<()> {
    tracing::info!("opening files");
    let jmdict = open(jmdict)?;
    let furigana = open(jmdict_furigana)?;
    let wfs = open(skeleton)?;

    tracing::info!("parsing jmdict version");
    let jmdict_version = parse_jmdict_version(&jmdict)?;

    tracing::info!("deserializing");
    let jmdict = JMdict::deserialize(jmdict)?;
    let furigana: Vec<jmdict_furigana::Furigana> =
        serde_json::from_reader(BufReader::new(furigana))?;
    let mut wfs: Wordfile = serde_json::from_reader(BufReader::new(wfs))?;

    tracing::info!("producing wordfile");
    wordfile::fill_skeleton(&mut wfs, version, jmdict, jmdict_version, furigana)?;

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

pub fn create_kanjifile_skeleton(
    kanjidic: &Path,
    jmdict: &Path,
    output: &Path,
) -> eyre::Result<()> {
    tracing::info!("opening files");
    let kd2 = open(kanjidic)?;
    let jmdict = open(jmdict)?;

    tracing::info!("deserializing files");
    let kd2: Kanjidic2 = serde_xml_rs::from_reader(BufReader::new(kd2))?;
    let jmdict = JMdict::deserialize(jmdict)?;

    tracing::info!("producing kanjifile skeleton");
    let skeleton = kanjifile_skeleton::create(kd2, jmdict)?;

    tracing::info!("writing output");
    let output = File::create(output)?;
    serde_json::to_writer_pretty(BufWriter::new(output), &skeleton)?;
    Ok(())
}

pub fn update_kanjifile_skeleton(
    kanjidic: &Path,
    jmdict: &Path,
    output: &Path,
) -> eyre::Result<()> {
    tracing::info!("opening files");
    let kd2 = open(kanjidic)?;
    let jmdict = open(jmdict)?;
    let kf = open(output)?;

    tracing::info!("deserializing files");
    let kd2: Kanjidic2 = serde_xml_rs::from_reader(BufReader::new(kd2))?;
    let jmdict = JMdict::deserialize(jmdict)?;
    let mut kf: Kanjifile = serde_json::from_reader(kf)?;

    tracing::info!("updating kanjifile skeleton");
    kanjifile_skeleton::update(&mut kf, kd2, jmdict)?;

    tracing::info!("writing output");
    let output = File::create(output)?;
    serde_json::to_writer_pretty(BufWriter::new(output), &kf)?;
    Ok(())
}

pub fn create_wordfile_skeleton(jmdict: &Path, output: &Path) -> eyre::Result<()> {
    tracing::info!("opening files");
    let jmdict = open(jmdict)?;

    tracing::info!("parsing jmdict version");
    let version = parse_jmdict_version(&jmdict)?;

    tracing::info!("deserializing");
    let jmdict = JMdict::deserialize(BufReader::new(jmdict))?;

    tracing::info!("producing wordfile skeleton");
    let skeleton = wordfile_skeleton::create(jmdict, version)?;

    tracing::info!("writing output");
    let output = File::create(output)?;
    serde_json::to_writer_pretty(BufWriter::new(output), &skeleton)?;
    Ok(())
}

pub fn update_wordfile_skeleton(jmdict: &Path, output: &Path) -> eyre::Result<()> {
    tracing::info!("opening files");
    let jmdict = open(jmdict)?;
    let wf = open(output)?;

    tracing::info!("parsing jmdict version");
    let version = parse_jmdict_version(&jmdict)?;

    tracing::info!("deserializing");
    let jmdict = JMdict::deserialize(BufReader::new(jmdict))?;
    let mut wf: Wordfile = serde_json::from_reader(wf)?;

    tracing::info!("updating wordfile skeleton");
    wordfile_skeleton::update(&mut wf, jmdict, version)?;

    tracing::info!("writing output");
    let output = File::create(output)?;
    serde_json::to_writer_pretty(BufWriter::new(output), &wf)?;
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
