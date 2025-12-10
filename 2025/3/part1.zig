const std = @import("std");

pub fn main() !void {
    std.debug.print("BEGIN\n", .{});
    var t = try std.time.Timer.start();

    var reader = std.io.getStdIn().reader();

    var result: usize = 0;
    var line_buf: [128]u8 = undefined;
    while (try reader.readUntilDelimiterOrEof(&line_buf, '\n')) |line| {
        var max_numbers = [_]u8{ 0, 0 };
        // const line_len = line.len;
        for (line, 0..) |c, idx| {
            const digit = c - '0';
            const current_number = max_numbers[0] * 10 + max_numbers[1];
            if (digit * 10 > current_number and idx < line.len - 1) {
                max_numbers[0] = digit;
                max_numbers[1] = 0;
            } else if (max_numbers[0] * 10 + digit > current_number) {
                max_numbers[1] = digit;
            }
        }

        result += max_numbers[0] * 10 + max_numbers[1];
    }

    std.debug.print("Result: {d}\n", .{result});
    std.debug.print("Took: {d}ms\n", .{@divFloor(t.read(), std.time.ns_per_ms)});
}
