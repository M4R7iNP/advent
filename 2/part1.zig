const std = @import("std");

pub fn main() !void {
    std.debug.print("BEGIN\n", .{});
    var t = try std.time.Timer.start();
    const stdin = std.io.getStdIn();
    const reader = stdin.reader();
    var line_buf: [256]u8 = undefined;
    var safe_lines_count: usize = 0;

    while (try reader.readUntilDelimiterOrEof(&line_buf, '\n')) |line| {
        var numbers: [8]i8 = undefined;
        var num_count: u8 = 0;
        var iter = std.mem.split(u8, line, " ");
        var is_safe = true;
        var is_increasing = false;

        while (iter.next()) |num_str| {
            numbers[num_count] = try std.fmt.parseInt(i8, num_str, 10);
            num_count += 1;
            if (num_count > 1) {
                const prev_num = numbers[num_count - 2];
                const curr_num = numbers[num_count - 1];
                if (num_count == 2) {
                    is_increasing = prev_num < curr_num;
                } else if ((prev_num < curr_num) != is_increasing) {
                    is_safe = false;
                    break;
                }
                const diff = @abs(try std.math.sub(i8, prev_num, curr_num));

                if ((diff == 0) or (diff > 3)) {
                    is_safe = false;
                    break;
                }
                // std.debug.print("{!}\n", .{diff});
            }
        }
        if (is_safe) {
            safe_lines_count += 1;
        }
    }

    std.debug.print("Result: {d}\n", .{safe_lines_count});
    std.debug.print("Took: {d}ms\n", .{@divFloor(t.read(), std.time.ns_per_ms)});
}
