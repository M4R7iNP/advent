#include <cassert>
#include <cstdio>
#include <iostream>
#include <vector>
#include <chrono>
#include <stdio.h>
#include <stddef.h>

using namespace std;

struct Machine {
    vector<bool> lights;
    vector<vector<uint8_t>> buttons; // each button toggles a set of lights
    vector<uint8_t> joltage_requirements;
};

char read_char_skip_whitespace() {
    char ch;
    do {
        ch = fgetc(stdin);
    } while (ch == ' ' || ch == '\n' || ch == '\t');
    return ch;
}

ostream& operator<<(ostream& os, const Machine& machine) {
    os << "[";
    for (auto light : machine.lights) {
        os << (light ? '#' : '.');
    }
    os << "] ";

    for (auto button : machine.buttons) {
        os << "(";
        for (auto toggle : button) {
            os << (int)toggle << ",";
        }
        os << ") ";
    }

    return os;
}

// brute-force search all button combinations
// assumes we never have to press the same button more than once
uint8_t find_button_combination(const Machine machine) {
    auto smallest = UINT8_MAX;
    uint32_t max_iterations = (1 << machine.buttons.size());

    for (uint32_t iteration = 0; iteration < max_iterations; iteration++) {
        auto lights = vector<bool>();
        lights.resize(machine.lights.size(), false);

        uint8_t button_index = 0;
        for (auto button : machine.buttons) {
            if (iteration & (1 << button_index++)) {
                for (auto light_to_toggle : button) {
                    lights[light_to_toggle] = !lights[light_to_toggle];
                }
            }
        }

        bool is_correct = equal(lights.begin(), lights.end(), machine.lights.begin());
        if (is_correct) {
            auto result = std::popcount(iteration);
            if (result < smallest) {
                smallest = result;
            }
        }
    }
    return smallest;
}

int main () {
    auto start = chrono::high_resolution_clock::now();
    unsigned long answer = 0;

    vector<Machine> machines;

    char ch;
    while ((ch = read_char_skip_whitespace()) == '[') {
        Machine machine;
        assert(ch == '[');
        while ((ch = read_char_skip_whitespace()) != ']') {
            machine.lights.push_back(ch == '#');
        }

        // parse series of (x,y) switch tuples
        while ((ch = read_char_skip_whitespace()) == '(') {
            uint8_t button_idx = 0;
            vector<uint8_t> button;

            while (true) {
                ch = read_char_skip_whitespace();

                // consume number
                assert(ch >= '0' && ch <= '9');
                uint8_t light_to_toggle = ch - '0';
                button.push_back(light_to_toggle);

                ch = read_char_skip_whitespace();
                if (ch == ',') {
                    continue;
                }
                if (ch == ')') {
                    break;
                }

                assert(false); // unreachable
            }

            assert(ch == ')');
            machine.buttons.push_back(button);
        }

        assert(ch == '{');
        while (true) {
            uint8_t joltage_requirement;
            scanf("%hhd", &joltage_requirement);
            machine.joltage_requirements.push_back(joltage_requirement);

            ch = read_char_skip_whitespace();
            if (ch == ',') {
                ch = read_char_skip_whitespace();
                continue;
            }

            if (ch == '}') {
                break;
            }
        }
        assert(ch == '}');

        cout << machine << endl;

        machines.push_back(machine);

        auto result = find_button_combination(machine);
        // cerr << "Result: " << (int) result << endl;
        answer += result;
    }

    cout << "Answer: " << answer << "!\n";
    cerr << "Took: "
         << chrono::duration_cast<chrono::milliseconds>(
             chrono::high_resolution_clock::now() - start)
         .count()
         << "ms" << endl;
}
