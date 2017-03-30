#include <stdio.h>
#include <stdlib.h>
#include <caseconv.h>

int main(int argc, char **argv) {
	for (int i = 1; i < argc; i++) {
		char *converted = caseconv_guess_and_convert(argv[i], CASECONV_CASE_TYPE_KEBAB);
		printf("%s\n", converted);
		free(converted);
	}
	return 0;
}
