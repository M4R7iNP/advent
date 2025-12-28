#include <algorithm>
#include <cmath>
#include <iostream>
#include <vector>
#include <chrono>
#include <boost/polygon/polygon.hpp>

#define LERP_FRAC 100

namespace bp = boost::polygon;
using namespace bp::operators;
using namespace std;
typedef bp::polygon_90_data<long> Polygon;
typedef bp::polygon_traits<Polygon>::point_type Point;
typedef bp::polygon_traits<Polygon>::coordinate_type Coord;

enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP,
};

int main() {
    auto start = chrono::high_resolution_clock::now();

    vector<Point> red_tiles;

    while (!feof(stdin)) {
        unsigned long x, y;
        if (scanf("%lu,%lu", &x, &y) != 2) {
            break;
        }
        Point red_tile(x, y);
        red_tiles.push_back(red_tile);
    }
    red_tiles.push_back(red_tiles[0]);

    cerr << "Generating polygon…";
    Polygon poly;
    boost::polygon::set_points(poly, red_tiles.begin(), red_tiles.end());
    cerr << " done!\n";
    cerr << "Took: "
         << chrono::duration_cast<chrono::milliseconds>(
             chrono::high_resolution_clock::now() - start)
         .count()
         << "ms" << endl;

    cerr << "Polygon area: " << bp::area(poly) << "\n";
    cerr << "Polygon perimeter: " << bp::perimeter(poly) << "\n";
    bp::rectangle_data<unsigned long> bounding_box;
    bp::extents(bounding_box, poly);
    cerr << "Polygon bounding box area: " << (bp::xl(bounding_box) - bp::xh(bounding_box) + 1) *
         (bp::yl(bounding_box) - bp::yh(bounding_box) + 1)
         << "\n";

    cerr << "Finding largest square…";
    unsigned long answer = 0;
    for (auto a = red_tiles.begin(); a != red_tiles.end(); a++) {
        auto b = a;
        advance(b, 1);
        for (; b != red_tiles.end(); b++) {
            unsigned long area = (abs((long)a->x() - (long)b->x()) + 1) *
                                 (abs((long)a->y() - (long)b->y()) + 1);

            // fast skip
            if (area < answer) {
                continue;
            }

            // check opposite corners
            Point xa_yb(a->x(), b->y());
            Point xb_ya(b->x(), a->y());

            if (!bp::contains(poly, xa_yb) || !bp::contains(poly, xb_ya)) {
                continue;
            }

            // go clock-wise around the rectangle, checking points along the edges
            bool is_outside = false;
            auto top_left = Point(min(a->x(), b->x()), max(a->y(), b->y()));
            auto top_right = Point(max(a->x(), b->x()), max(a->y(), b->y()));
            auto bottom_left = Point(min(a->x(), b->x()), min(a->y(), b->y()));
            auto bottom_right = Point(max(a->x(), b->x()), min(a->y(), b->y()));
            auto x = top_left.x();
            auto y = top_left.y();
            for (auto direction = 0; direction < 4; direction++) {
                for (double i = 0; i < LERP_FRAC; i++) {
                    if (direction == RIGHT) {
                        x = lerp(top_left.x(), top_right.x(), i / LERP_FRAC);
                    } else if (direction == DOWN) {
                        y = lerp(top_right.y(), bottom_right.y(), i / LERP_FRAC);
                    } else if (direction == LEFT) {
                        x = lerp(bottom_right.x(), bottom_left.x(), i / LERP_FRAC);
                    } else if (direction == UP) {
                        y = lerp(bottom_left.y(), top_left.y(), i / LERP_FRAC);
                    }

                    if (!bp::contains(poly, Point((Coord) x, (Coord) y))) {
                        is_outside = true;
                        break;
                    }
                }
                if (is_outside) {
                    break;
                }
            }

            if (is_outside) {
                continue;
            }

            if (area > answer) {
                answer = area;
            }
        }
    }
    cerr << " done!\n";

    cout << "Answer: " << answer << "!\n";
    cerr << "Took: "
         << chrono::duration_cast<chrono::milliseconds>(
             chrono::high_resolution_clock::now() - start)
         .count()
         << "ms" << endl;
}
