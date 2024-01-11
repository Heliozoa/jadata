cargo run --release --\
    kanjifile\
        -d ./external/kanjidic2.xml\
        -k ./external/kradfile\
        -s ./included/kanjifile_skeleton.json\
        -t postcard\
        -o ./generated/kanjifile.postcard
cargo run --release -- \
    wordfile\
        -j ./external/JMdict_e_examp.xml\
        -f ./external/JmdictFuriganaPretty.json\
        -s ./included/wordfile_skeleton.json\
        -t postcard\
        -o ./generated/wordfile.postcard
