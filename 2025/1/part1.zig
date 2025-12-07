const std = @import("std");

pub fn main() !void {
    std.debug.print("BEGIN\n", .{});
    var t = try std.time.Timer.start();

    var reader = std.io.getStdIn().reader();

    var result: usize = 0;
    var current: isize = 50;
    var line_buf: [8]u8 = undefined;
    while (try reader.readUntilDelimiterOrEof(&line_buf, '\n')) |line| {
        const direction = line[0];
        const count = try std.fmt.parseInt(isize, line[1..], 10);
        switch (direction) {
            'L' => current -= count,
            'R' => current += count,
            else => unreachable,
        }
        current = @mod(current, 100);
        if (current == 0) {
            result += 1;
        }
    }

    std.debug.print("Result: {d}\n", .{result});
    std.debug.print("Took: {d}ms\n", .{@divFloor(t.read(), std.time.ns_per_ms)});
}
