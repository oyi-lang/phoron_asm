files=`ls *.pho`

make -s clean
for file in $files
do
  make -s all
  echo -n "$file: "
  ./phoron < $file
done
