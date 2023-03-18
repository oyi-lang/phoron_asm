#!/usr/bin/env bash

GREEN="\033[32m"
RED="\033[31m"
RESET="\033[0m"

jasmin_files=$( ls *.j )

for jasmin_file in ${jasmin_files}
do
  cargo run --release -- -f ${jasmin_file} &> /dev/null

  if [[ $? -eq 0 ]]
  then
    echo -e "Processing ${jasmin_file}: \033[32mPASSED\033[0m"
  else
    echo -e "Processing ${jasmin_file}: \033[31mFAILED\033[0m"
  fi
done

