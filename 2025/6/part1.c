#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>

char consume_spaces(FILE* input) {
    char ch;
    while ((ch = fgetc(input)) != EOF) {
        if (ch != ' ') {
            ungetc(ch, input);
            return ch;
        }
    }
    return ch;
}

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

    struct MARTIN_ARRAY add_array = new_MARTIN_ARRAY();
    struct MARTIN_ARRAY mul_array = new_MARTIN_ARRAY();

    // collect first numbers
    while (1) {
        uint value;
        if (fscanf(input, "%d", &value) != 1) {
            break;
        }
        // fprintf(stderr, "%d", value);
        push_MARTIN_ARRAY(&add_array, value);
        push_MARTIN_ARRAY(&mul_array, value);
        if (consume_spaces(input) == '\n') {
            // fprintf(stderr, "\n");
            break;
        }
        // fprintf(stderr, " ");
    }

    char newline_char = fgetc(input);
    assert(newline_char == '\n');

    fprintf(stderr, "Array length: %d, size: %d\n", add_array.length, add_array.size);

    // add and multiply in respective arrays
    while (1) {
        consume_spaces(input);
        for (uint i = 0; i < add_array.length; i++) {
            uint value;
            if (fscanf(input, "%d", &value) != 1) {
                break;
            }
            // fprintf(stderr, "%d", value);
            add_array.data[i] += value;
            mul_array.data[i] *= value;
            consume_spaces(input);
            // fprintf(stderr, " ");
        }
        // fprintf(stderr, "\n");
        char next_char = consume_spaces(input);
        if (next_char == '*' || next_char == '+') {
            break;
        }
    }

    // consume_spaces(input);
    uint64_t result = 0;

    // collect symbols at the end
    for (uint i = 0; i < add_array.length; i++) {
        char symbol = fgetc(input);
        if (symbol == '+') {
            result += add_array.data[i];
        } else if (symbol == '*') {
            result += mul_array.data[i];
        }
        consume_spaces(input);
        if (feof(input)) {
            break;
        }
    }

    free(add_array.data);
    free(mul_array.data);

    fprintf(stderr, "Result: %lu\n", result);
}
