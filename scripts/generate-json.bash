#!/bin/bash

# Generates the complete files in the JSON format

cargo run --release --\
    kanjifile\
        -d ./external/kanjidic2.xml\
        -k ./external/kradfile\
        -s ./included/kanjifile_skeleton.json\
        -t json\
        -o ./generated/kanjifile.json
#cargo run --release -- \
    #wordfile\
        #-j ./external/JMdict_e_examp.xml\
        #-f ./external/JmdictFuriganaPretty.json\
        #-s ./included/wordfile_skeleton.json\
        #-t json\
        #-o ./generated/wordfile.json
