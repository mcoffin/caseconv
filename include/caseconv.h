#ifndef __CASECONV_H__
#define __CASECONV_H__

typedef enum {
	CASECONV_CASE_TYPE_CAMEL,
	CASECONV_CASE_TYPE_SNAKE,
	CASECONV_CASE_TYPE_KEBAB
} caseconv_case_type_t;

char *caseconv_convert_case(const char *src, caseconv_case_type_t src_type, caseconv_case_type_t dst_type);
char *caseconv_unjumble(const char *src, caseconv_case_type_t dst_type);
caseconv_case_type_t caseconv_guess_case(const char *src);
char *caseconv_guess_and_convert(const char *src, caseconv_case_type_t dst_type);

#endif
