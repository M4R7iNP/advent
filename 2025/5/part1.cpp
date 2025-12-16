#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <forward_list>
#include <iostream>
#include <tuple>
#include <chrono>

int main() {
    auto start = std::chrono::high_resolution_clock::now();
    char line[36];
    unsigned int result = 0;

    std::forward_list<std::tuple<std::uint64_t, std::uint64_t>> ranges;

    while (fgets(line, sizeof(line), stdin) != NULL) {
        if (line[0] == '\n') {
            break;
        }
        std::uint64_t start;
        std::uint64_t end;
        sscanf(line, "%lu-%lu", &start, &end);
        ranges.push_front({ start, end });
    }

    while (fgets(line, sizeof(line), stdin) != NULL) {
        std::uint64_t value;
        sscanf(line, "%lu", &value);

        for (auto range : ranges) {
            if (value >= std::get<0>(range) && value <= std::get<1>(range)) {
                result++;
                break;
            }
        }
    }

    std::cerr << "Result: " << result << std::endl;
    std::cerr << "Took: "
              << std::chrono::duration_cast<std::chrono::milliseconds>(
                     std::chrono::high_resolution_clock::now() - start)
                     .count()
              << "ms" << std::endl;
}
