#!/bin/bash

# Updates the skeleton files

cargo run --release --\
    kanjifile-skeleton\
        -d ./external/kanjidic2.xml\
        -o ./included/kanjifile_skeleton.json
cargo run --release -- \
    wordfile-skeleton\
        -j ./external/JMdict_e_examp.xml\
        -o ./included/wordfile_skeleton.json
