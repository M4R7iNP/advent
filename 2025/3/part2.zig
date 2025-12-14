const std = @import("std");

pub fn main() !void {
    std.debug.print("BEGIN\n", .{});
    var t = try std.time.Timer.start();

    var reader = std.io.getStdIn().reader();

    var result: u64 = 0;
    var line_buf: [128]u8 = undefined;
    while (try reader.readUntilDelimiterOrEof(&line_buf, '\n')) |line| {
        var max_number_digits: [12]u64 = .{0} ** 12;
        var max_number: u64 = 0;
        for (line) |c| {
            const digit = c - '0';

            // std.debug.print("NUMBER_TO_BEAT: {d} (with digit {d})\n", .{ max_number, digit });

            var biggest_candidate: u64 = max_number;
            var biggest_candidate_digits: [12]u64 = max_number_digits;
            for (0..12) |idx_to_drop| {
                var test_digits: [12]u64 = .{0} ** 12;
                var test_number: u64 = 0;
                for (0..11) |i_a| {
                    var i_b = i_a;
                    if (i_b >= idx_to_drop) {
                        i_b = i_b + 1;
                    }
                    test_digits[i_a] = max_number_digits[i_b];
                    test_number *= 10;
                    test_number += test_digits[i_a];
                }
                test_digits[11] = digit;
                test_number *= 10;
                test_number += digit;
                // std.debug.print("CHECKING IF {d} > {d}\n", .{ test_number, biggest_candidate });
                if (test_number > biggest_candidate) {
                    biggest_candidate = test_number;
                    biggest_candidate_digits = test_digits;
                }
            }
            const test_digits = biggest_candidate_digits;
            const test_number = biggest_candidate;
            // std.debug.print("TEST NUMBER: {d}\n", .{test_number});

            if (test_number > max_number) {
                max_number_digits = test_digits;
                max_number = test_number;
            }
        }

        // std.debug.print("ENDED UP WITH NUMBER: {d}\n", .{max_number});
        result += max_number;
    }

    std.debug.print("Result: {d}\n", .{result});
    std.debug.print("Took: {d}ms\n", .{@divFloor(t.read(), std.time.ns_per_ms)});
}
