#include <stdio.h>
#include <stdlib.h>
#include <caseconv.h>

int main(int argc, char **argv) {
	char *converted = caseconv_convert_case(argv[1], CASECONV_CASE_TYPE_CAMEL, CASECONV_CASE_TYPE_KEBAB);
	printf("%s\n", converted);
	free(converted);
	return 0;
}
