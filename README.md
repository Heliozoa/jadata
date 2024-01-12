# `jadata`
Generates the kanjifile and wordfile files used by [lbr](https://github.com/Heliozoa/lbr).

Both files are derived from "skeleton" files that contain stable ids mapped to each entry. The skeletons are then filled with data from the following files:

`kanjifile_skeleton.json`:
- [KANJIDIC2](https://www.edrdg.org/wiki/index.php/KANJIDIC_Project) (`kanjidic2.xml`) from The Electronic Dictionary Research and Development Group. Contains a list of kanji and their meanings.
- [KRADFILE](https://www.edrdg.org/krad/kradinf.html) (`kradfile`) from the The Electronic Dictionary Research and Development Group. Contains decompositions of each kanji into common "components".

`wordfile_skeleton.json`:
- [JMdict](https://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project) (`JMdict_e_examp.xml`) from The Electronic Dictionary Research and Development Group. Contains a list of words and phrases, their readings and meanings.
- [JmdictFurigana](https://github.com/Doublevil/JmdictFurigana) (`JmdictFurigana.json`) from Doublevil. Contains the readings for each word in JMdict assigned as furigana.

The core concept is that the kanjifile and wordfile can easily be updated both from new versions of KANJIDIC2 and JMdict, as well as with manual updates for the needs of `jadata` such as kanji names and the list of similar kanji by updating the skeleton. This way it's not necessary to store the large, complete files in version control.


## Differences from JMdict
jadata's definition of a "word" is a little different from JMdict's. Essentially, jadata prioritises the "written form" of the word in order to make things easier for a Japanese learner, whereas JMdict prioritises the "meaning" of the word as a dictionary would.

For example, in jadata 船 and 舟 are two different words, both meaning ship and both read ふね, whereas in JMdict they are both grouped as two different ways to write the same word. When learning Japanese, you would have to learn each written form separately, and so jadata considers them their own individual words.


## Crates
### jadata_cli
A binary crate that implements functionality for generating and updating the `kanjifile.json` and `wordfile.json` files.

### jadata
A library crate which contains the `Kanjifile` and `Wordfile` data structures and logic for serializing and deserializing them.


## Updating the skeletons
See the files in the `scripts` directory, or use the CLI manually with `cargo run`. The wordfile is large so updating it may take a moment.


## License
jadata's code is licensed under MPL-2.0.

The files in `./included` as well as the files created by the program are licensed under CC BY-SA 4.0, matching the license the generated files are derived from.
