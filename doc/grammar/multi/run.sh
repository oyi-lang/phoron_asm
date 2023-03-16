#!/usr/bin/env bash

src_files=`ls *.pho`

cargo build --release

for f in ${src_files}
do
  cargo run --release -- -f $f
done

java -cp . Main

rm *.class