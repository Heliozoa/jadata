#!/bin/bash

# Generates the complete files in the JSON format

echo "Input the version for the kanjifile:"
read -r kanjifile_version

echo "Input the version for the wordfile:"
read -r wordfile_version

cargo run --release --\
    kanjifile\
        -v "$kanjifile_version"\
        -d ./external/kanjidic2.xml\
        -k ./external/kradfile\
        -s ./included/kanjifile_skeleton.json\
        -t json\
        -o ./generated/kanjifile.json
cargo run --release -- \
    wordfile\
        -v "$wordfile_version"\
        -j ./external/JMdict_e_examp.xml\
        -f ./external/JmdictFuriganaPretty.json\
        -s ./included/wordfile_skeleton.json\
        -t json\
        -o ./generated/wordfile.json
