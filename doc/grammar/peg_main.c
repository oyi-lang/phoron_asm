#include <stdio.h>

extern int yyparse();

int main() {
  if (yyparse()) {
    fprintf(stdout, "PASSED\n");
  } else {
    fprintf(stderr, "FAILED\n");
  };
  return 0;
}