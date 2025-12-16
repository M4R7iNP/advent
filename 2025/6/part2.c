#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

struct MARTIN_ARRAY {
    unsigned long* data;
    int length;
    int size;
};

struct MARTIN_ARRAY new_MARTIN_ARRAY() {
    return (struct MARTIN_ARRAY) {
        .length = 0, .size = 8, .data = malloc(sizeof(unsigned long) * 8)
    };
}
void increase_MARTIN_ARRAY(struct MARTIN_ARRAY *arr) {
    arr->size *= 2;
    arr->data = realloc(arr->data, sizeof(unsigned long) * arr->size);
}

void push_MARTIN_ARRAY(struct MARTIN_ARRAY *arr, unsigned long value) {
    arr->data[arr->length++] = value;
    if (arr->length >= arr->size) {
        increase_MARTIN_ARRAY(arr);
    }
}

int main() {
    FILE* input = stdin;

    struct MARTIN_ARRAY numbers = new_MARTIN_ARRAY();

    // collect first numbers
    while (1) {
        char ch = fgetc(input);
        if (ch == '\n') {
            break;
        }
        uint value = 0;
        if (ch != ' ') {
            sscanf(&ch, "%d", &value);
        }

        push_MARTIN_ARRAY(&numbers, value);
    }

    fprintf(stderr, "Array length: %d, size: %d\n", numbers.length, numbers.size);

    // add and multiply in respective arrays
    uint processing_numbers = 1;
    while (1) {
        for (uint i = 0; i < numbers.length; i++) {
            char ch = fgetc(input);
            if (ch == '*' || ch == '+') {
                ungetc(ch, input);
                processing_numbers = 0;
                break;
            }
            uint value = 0;
            if (ch != ' ') {
                sscanf(&ch, "%d", &value);
            }

            if (value > 0) {
                numbers.data[i] *= 10;
                numbers.data[i] += value;
            }
        }

        if (!processing_numbers) {
            break;
        }

        char newline_char = fgetc(input);
        assert(newline_char == '\n');
    }

    // for (uint i = 0; i < numbers.length; i++) {
    //     fprintf(stderr, "NUMBER: %ld\n", numbers.data[i]);
    // }

    unsigned long result = 0;

    // collect symbols at the end
    char current_symbol;
    unsigned long acc = 0;
    for (uint i = 0; i < numbers.length; i++) {
        char symbol = fgetc(input);
        unsigned long value = numbers.data[i];
        if (value == 0) {
            // fprintf(stderr, "Adding %ld to result\n", acc);
            result += acc;
            acc = 0;
            continue;
        }

        if (symbol == '+') {
            current_symbol = symbol;
        } else if (symbol == '*') {
            current_symbol = symbol;
        }

        if (current_symbol == '+') {
            acc += value;
        } else if (current_symbol == '*') {
            if (acc == 0) {
                acc = 1;
            }
            acc *= value;
        }
    }

    // fprintf(stderr, "Adding %ld to result\n", acc);
    result += acc;

    fprintf(stderr, "Result: %lu\n", result);
}
