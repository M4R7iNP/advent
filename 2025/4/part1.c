#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LINE_BUFFER_SIZE 256

int main() {
    char prev_line[LINE_BUFFER_SIZE];
    char curr_line[LINE_BUFFER_SIZE];
    char next_line[LINE_BUFFER_SIZE];

    fgets(curr_line, LINE_BUFFER_SIZE, stdin);
    fgets(next_line, LINE_BUFFER_SIZE, stdin);

    int result = 0;
    while (1) {
        if (curr_line[0] == '\0') {
            break;
        }

        for (int idx = 0; curr_line[idx] != '\0' && curr_line[idx] != '\n'; idx++) {
            if (curr_line[idx] != '@') {
                printf(".");
                continue;
            }
            int around = 0;
            for (int bidx = idx - 1; bidx <= idx + 1; bidx++) {
                if (bidx < 0) continue;
                if (prev_line[bidx] == '@') around++;
            }
            if (idx > 0 && curr_line[idx - 1] == '@') around++;
            if (curr_line[idx + 1] == '@') around++;
            for (int nidx = idx - 1; nidx <= idx + 1; nidx++) {
                if (nidx < 0) continue;
                if (next_line[nidx] == '@') around++;
            }
            if (around < 4) {
                printf("x");
                result++;
            } else {
                printf("@");
            }
        }

        printf("\n");

        strcpy(prev_line, curr_line);
        strcpy(curr_line, next_line);
        if (fgets(next_line, LINE_BUFFER_SIZE, stdin) == NULL) {
            memset(next_line, 0, LINE_BUFFER_SIZE);
        }
    }

    fprintf(stderr, "Result: %d\n", result);
}
