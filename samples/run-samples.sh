#!/usr/bin/env bash

phoron_files=$( ls *.pho  ls mak_wcai/*.pho )

for phoron_file in ${phoron_files}
do
  cargo run --release -- -f ${phoron_file} &> /dev/null

  if [[ $? -eq 0 ]]
  then
    printf "Processing ${phoron_file}: %b\n" "\033[32mPASSED\033[0m"
  else
    printf "Processing ${phoron_file}: %b\n" "\033[31mFAILED\033[0m"
  fi

  rm  -f *.class
  rm -f mak_wcai/*.class
done

