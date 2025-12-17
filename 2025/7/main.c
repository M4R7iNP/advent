#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define LINE_BUF_SIZE 256
#define ACCUMULATOR_T = unsigned long

int main() {
    FILE* input = stdin;

    char prev_line[LINE_BUF_SIZE] = "";
    char curr_line[LINE_BUF_SIZE] = "";

    fgets(curr_line, LINE_BUF_SIZE, input);
    int line_len = strlen(curr_line);

    unsigned long * acc_arr = malloc(sizeof(unsigned long) * line_len);
    memset(acc_arr, 0, sizeof(unsigned long) * line_len);

    printf("%s", curr_line);

    unsigned int result = 0;
    unsigned int col = 0;
    unsigned int line_idx = 0;
    while (1) {
        memcpy(&prev_line, &curr_line, LINE_BUF_SIZE);
        memset(&curr_line, 0, LINE_BUF_SIZE);
        if (fgets(curr_line, LINE_BUF_SIZE, input) == NULL) {
            break;
        }

        // first, draw beams
        for (int col = 0; col < line_len; col++) {
            if (curr_line[col] == '.') {
                switch (prev_line[col]) {
                case 'S':
                    acc_arr[col] = 1;
                case '|':
                    curr_line[col] = '|';
                }
            }
        }

        // next, draw and count splits
        for (int col = 0; col < line_len; col++) {
            if (curr_line[col] == '^' && prev_line[col] == '|') {
                curr_line[col - 1] = '|';
                curr_line[col + 1] = '|';
                result++;

                // part2 logic – based on https://www.reddit.com/r/adventofcode/comments/1pgbg8a/2025_day_7_part_2_visualization_for_the_sample/
                unsigned long value = acc_arr[col];
                acc_arr[col - 1] += value;
                acc_arr[col + 1] += value;
                acc_arr[col] = 0;
            }
        }

        printf("%s", curr_line);
    }

    unsigned long result_part2 = 0;
    for (int i = 0; i < line_len; i++) {
        result_part2 += acc_arr[i];
    }

    fprintf(stderr, "Result part1: %d\n", result);
    fprintf(stderr, "Result part2: %ld\n", result_part2);
    return 0;
}
