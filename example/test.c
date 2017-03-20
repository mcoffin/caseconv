#include <stdio.h>
#include <stdlib.h>
#include <caseconv.h>

static char *convert(const char *source, caseconv_case_type_t dst_type) {
	caseconv_case_type_t source_type = caseconv_guess_case(source);
	return caseconv_convert_case(source, source_type, dst_type);
}

int main(int argc, char **argv) {
	char *converted = convert(argv[1], CASECONV_CASE_TYPE_KEBAB);
	printf("%s\n", converted);
	free(converted);
	return 0;
}
