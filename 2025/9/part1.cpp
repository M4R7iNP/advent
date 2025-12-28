#include <iostream>
#include <vector>
#include <chrono>

struct tile {
    unsigned long x;
    unsigned long y;
};

int main() {
    auto start = std::chrono::high_resolution_clock::now();

    std::vector<tile> red_tiles;

    while (!feof(stdin)) {
        tile red_tile;
        if (scanf("%lu,%lu", &red_tile.x, &red_tile.y) != 2) {
            break;
        }
        red_tiles.push_back(red_tile);
    }

    unsigned long answer = 0;
    for (auto a = red_tiles.begin(); a != red_tiles.end(); a++) {
        auto b = a;
        std::advance(b, 1);
        for (; b != red_tiles.end(); b++) {
            unsigned long area = (std::abs((long)a->x - (long)b->x) + 1) *
                                 (std::abs((long)a->y - (long)b->y) + 1);
            if (area > answer) {
                answer = area;
            }
        }
    }


    std::cout << "Answer: " << answer << "!\n";
    std::cerr << "Took: "
              << std::chrono::duration_cast<std::chrono::milliseconds>(
                  std::chrono::high_resolution_clock::now() - start)
              .count()
              << "ms" << std::endl;
}
