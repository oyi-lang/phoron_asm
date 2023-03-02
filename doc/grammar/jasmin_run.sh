#!/usr/bin/env bash

if [ $# -eq 0 ]
then
  echo "Usage: run_jasmin.sh <source-file>"
  exit 1
fi

sourcefile="$1"
classfile=${sourcefile%.*}

java jasmin.Main ${sourcefile} && java -cp . ${classfile}
rm -f ${classfile}.class
