The Pascal (`.pas`) source files are taken from the following source: https://github.com/GunterMueller/Mak_Writing_Compilers_and_Interpreters/tree/master/Mak_Writing_Compiler_3rd_Ed/Chapter18. The given repository is the source code repository
for the book, ["Writing compilers and Interpreters" by Ronald Mak.](https://www.wiley.com/en-us/Writing+Compilers+and+Interpreters%3A+A+Software+Engineering+Approach%2C+3rd+Edition-p-9780470177075). These source files are simply given for reference. 
The actual items of interested are the generated Jasmin source files, which are described next.

The Jasmin (`.j`) source files were generated from the Pascal source files (if interested in generating them yourself, follow the instructons over at http://web.archive.org/web/20180825124453/http://www.apropos-logic.com/wci/). The Jasmin files are
provided here to act as a handy reference for Jasmin syntax and semantics (especially since examples are scarce on the internet). The secondary goal was to act as testing materials for `Phoron`.

## Build and Run

```
  $ chmod a+x run-samples.sh
  $ ./run-samples.sh

```

Note: This script simply ensures that the given Jasmin source file is parsed correctly, and that the `.class` file is generated correctly, The generated `class` file depends on runtime elements outside this project's scope (these can be found in the link given above).
