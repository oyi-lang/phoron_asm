#!/usr/bin/env bash

test_files=$( ls ../../samples/*.pho ../../samples/mak_wcai/*.pho  )

make -s clean ; make -s all

for test_file in $test_files
do
  ./peg_phoron < $test_file &> /dev/null
  
  if [[ $? -eq 0 ]]
  then
    printf "${test_file}: %b\n" "\033[32mPASSED\033[0m"
  else
    printf "${test_file}: %b\n" "\033[33mFAILED\033[0m"
  fi
done
