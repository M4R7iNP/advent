#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <iostream>
#include <forward_list>
#include <tuple>
#include <chrono>

#define NUMBER_T std::uint64_t
#define TUPLE_T std::tuple<NUMBER_T, NUMBER_T>
#define LIST_T std::forward_list<TUPLE_T>

int main() {
    auto start = std::chrono::high_resolution_clock::now();
    char line[36];
    NUMBER_T result = 0;

    LIST_T fresh_ranges;

    while (fgets(line, sizeof(line), stdin) != NULL) {
        if (line[0] == '\n') {
            break;
        }
        NUMBER_T start, end;
        if (sscanf(line, "%lu-%lu", &start, &end) != 2) {
            perror("Failed to parse range");
        }
        fresh_ranges.push_front({ start, end });
    }

    fclose(stdin); // skip the rest of the input

    // merge the ranges in multiple passes
    LIST_T fresh_ranges_unmerged = fresh_ranges;
    LIST_T fresh_ranges_merged;
    bool merged_any = false;
    do {
        fresh_ranges_merged = {};
        merged_any = false;
        for (auto range : fresh_ranges_unmerged) {
            bool merged = false;
            for (auto &merged_range : fresh_ranges_merged) {
                if (std::get<0>(range) <= std::get<1>(merged_range) + 1 &&
                        std::get<1>(range) >= std::get<0>(merged_range) - 1)
                {
                    std::get<0>(merged_range) = std::min(std::get<0>(merged_range), std::get<0>(range));
                    std::get<1>(merged_range) = std::max(std::get<1>(merged_range), std::get<1>(range));
                    merged = true;
                    merged_any = true;
                }
            }
            if (!merged) {
                fresh_ranges_merged.push_front(range);
            }
        }
        fresh_ranges_unmerged = fresh_ranges_merged;
    } while (merged_any);

    // count each ingredient id in the merged fresh ranges
    for (auto range : fresh_ranges_merged) {
        NUMBER_T count = std::get<1>(range) - std::get<0>(range) + 1;
        result += count;
    }

    std::cerr << "Result: " << result << std::endl;
    std::cerr << "Took: "
              << std::chrono::duration_cast<std::chrono::milliseconds>(
                  std::chrono::high_resolution_clock::now() - start)
              .count()
              << "ms" << std::endl;
}
