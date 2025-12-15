#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/mman.h>

#define LINE_BUFFER_SIZE 256

int process(FILE* input, FILE* output) {
    char prev_line[LINE_BUFFER_SIZE];
    char curr_line[LINE_BUFFER_SIZE];
    char next_line[LINE_BUFFER_SIZE];

    fgets(curr_line, LINE_BUFFER_SIZE, input);
    fgets(next_line, LINE_BUFFER_SIZE, input);

    int result = 0;
    while (1) {
        if (curr_line[0] == '\0') {
            break;
        }

        for (int idx = 0; curr_line[idx] != '\0' && curr_line[idx] != '\n'; idx++) {
            if (curr_line[idx] != '@') {
                fprintf(output, ".");
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
                fprintf(output, "x");
                result++;
            } else {
                fprintf(output, "@");
            }
        }

        fprintf(output, "\n");

        strcpy(prev_line, curr_line);
        strcpy(curr_line, next_line);
        if (fgets(next_line, LINE_BUFFER_SIZE, input) == NULL) {
            memset(next_line, 0, LINE_BUFFER_SIZE);
        }
    }

    return result;
}

int main() {
    int result = 0;
    clock_t start = clock();

    FILE* input = stdin;
    FILE* output = tmpfile();

    while (1) {
        int current_result = process(input, output);
        result += current_result;
        fclose(input);
        if (current_result == 0) {
            break;
        }

        freopen(NULL, "r+", output);
        input = output;
        output = tmpfile();
    }

    fclose(output);

    fprintf(stderr, "Result: %d\n", result);
    fprintf(stderr, "Took: %.2fms\n", (float)(clock() - start) / CLOCKS_PER_SEC * 1000.0);
}
