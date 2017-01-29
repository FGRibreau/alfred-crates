#!/bin/bash
OUTPUT_FILE=alfred-crates

clean (){
  rm -v -rf release
  rm -v -rf *.alfredworkflow
}

clean;

mkdir -p release{/,/target/release}
cargo build --release

# zip it
zip -9 -r $OUTPUT_FILE.alfredworkflow icon.png info.plist target/release/alfred-crates
