#include <regex.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef unsigned long MARTIN_NUMBER;

int main() {
    const char *filename = "input.txt";
    regex_t regex;
    FILE *file;
    char line[1024];
    MARTIN_NUMBER result = 0;

    if (regcomp(&regex, "^\\([0-9]\\+\\)\\1\\+$", 0) != 0) {
        fprintf(stderr, "Could not compile regex\n");
        return 1;
    }

    file = fopen(filename, "r");
    if (file == NULL) {
        fprintf(stderr, "Could not open file %s\n", filename);
        regfree(&regex);
        return 1;
    }

    char *saveptr;

    fgets(line, sizeof(line), file);
    char *token;
    for (token = strtok_r(line, ",", &saveptr); token != NULL; token = strtok_r(NULL, ",", &saveptr)) {
        // printf("Token: %s\n", token);
        MARTIN_NUMBER first_num, second_num;
        sscanf(token, "%lu-%lu", &first_num, &second_num);

        printf("First num: %lu, Second num: %lu\n", first_num, second_num);
        for (MARTIN_NUMBER i = first_num; i <= second_num; i++) {
            char num[16];
            sprintf(num, "%lu", i);
            if (regexec(&regex, num, 0, NULL, 0) == 0) {
                result += i;
            }
        }
    }

    fclose(file);
    regfree(&regex);

    printf("Result: %lu\n", result);
    return 0;
}

