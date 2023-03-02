#!/usr/bin/env bash

if [ $# -eq 0 ]
then
  echo "Usage: phoron_run.sh <source-file>"
  exit 1
fi

sourcefile="$1"
classfile=${sourcefile%.*}

cargo run --release ${sourcefile} && java -cp . ${classfile}
