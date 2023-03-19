.PHONY: all clean

all:
	peg -o peg_phoron.c phoron.peg  && gcc -o peg_phoron peg_phoron.c peg_main.c

clean:
	rm -f peg_phoron peg_phoron.c *.class