CC ?= gcc

SOURCES = $(shell find . -type f -name '*.c')

OBJECTS = $(SOURCES:%.c=%.o)

CFLAGS += -I../include

LDFLAGS += -L../target/release -lcaseconv

EXECUTABLE_NAME ?= to-kebab

.PHONY: default clean caseconv

default: $(EXECUTABLE_NAME)

clean:
	-rm -f $(OBJECTS)
	-rm -f $(EXECUTABLE_NAME)

%.o: %.c
	$(CC) $(CFLAGS) -c $<

$(EXECUTABLE_NAME): $(OBJECTS)
	$(CC) $(LDFLAGS) $^ -o $@
