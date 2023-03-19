#!/usr/bin/env bash

src_files=`ls *.pho`

for f in ${src_files}
do
  cargo run --release -- -f $f &>/dev/null
done

java -cp . Main

rm *.class