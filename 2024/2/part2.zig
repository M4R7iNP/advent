const std = @import("std");

fn test_numbers(numbers: [8]i8, numbers_len: u8, errors: *[16]usize, errors_len: *u8) !void {
    var is_increasing = false;
    for (1..numbers_len) |idx| {
        const prev_num = numbers[idx - 1];
        const curr_num = numbers[idx];
        if (idx == 1) {
            is_increasing = prev_num < curr_num;
        } else if ((prev_num < curr_num) != is_increasing) {
            errors[errors_len.*] = idx - 1;
            errors_len.* += 1;
            errors[errors_len.*] = idx;
            errors_len.* += 1;
            continue;
        }
        const diff = @abs(try std.math.sub(i8, prev_num, curr_num));

        if ((diff == 0) or (diff > 3)) {
            errors[errors_len.*] = idx - 1;
            errors_len.* += 1;
            errors[errors_len.*] = idx;
            errors_len.* += 1;
            continue;
        }
    }
}

pub fn main() !void {
    std.debug.print("BEGIN\n", .{});
    var t = try std.time.Timer.start();
    const stdin = std.io.getStdIn();
    const reader = stdin.reader();
    var line_buf: [256]u8 = undefined;
    var safe_lines_count: usize = 0;

    while (try reader.readUntilDelimiterOrEof(&line_buf, '\n')) |line| {
        var numbers: [8]i8 = undefined;
        var numbers_len: u8 = 0;
        var iter = std.mem.split(u8, line, " ");

        while (iter.next()) |num_str| {
            numbers[numbers_len] = try std.fmt.parseInt(i8, num_str, 10);
            numbers_len += 1;
        }

        var errors2: [16]usize = undefined;
        var errors_len2: u8 = 0;
        try test_numbers(numbers, numbers_len, &errors2, &errors_len2);
        // const errors: [16]usize = .{ 0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0 };
        // const errors_len: u8 = 8;
        var is_safe = errors_len2 == 0;
        if (!is_safe) {
            const errors: [16]usize = .{ 0, 1, 2, 3, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0 };
            const errors_len: u8 = 8;
            for (0..errors_len) |idx| {
                const idx_to_remove = errors[idx];
                var numbers_to_try: [8]i8 = undefined;
                // @memset(&numbers_to_try, 0);
                var i: u8 = 0;
                for (0..numbers_len) |ii| {
                    if (ii == idx_to_remove) {
                        continue;
                    }
                    numbers_to_try[i] = numbers[ii];
                    i += 1;
                }
                // std.mem.copyForwards(i8, &numbers_to_try, numbers[0..(idx_to_remove - 1)]);
                // if (idx_to_remove != numbers_len - 1) {
                //     std.mem.copyForwards(i8, &numbers_to_try[(idx_to_remove - 1)..], numbers[idx_to_remove..(numbers_len - 1)]);
                // }
                var new_errors_count: u8 = 0;
                var new_errors: [16]usize = undefined;
                try test_numbers(numbers_to_try, i, &new_errors, &new_errors_count);
                if (new_errors_count == 0) {
                    is_safe = true;
                    break;
                }
            }
        }

        if (is_safe) {
            safe_lines_count += 1;
        }
    }

    std.debug.print("Result: {d}\n", .{safe_lines_count});
    std.debug.print("Took: {d}ms\n", .{@divFloor(t.read(), std.time.ns_per_ms)});
}
