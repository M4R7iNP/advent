const std = @import("std");

pub fn main() !void {
    std.debug.print("BEGIN\n", .{});
    var t = try std.time.Timer.start();

    var reader = std.io.getStdIn().reader();

    var stopped_at_zero_count: usize = 0;
    var crossed_zero_count: usize = 0;

    var current: isize = 50;
    var line_buf: [8]u8 = undefined;
    // std.debug.print("The dial starts by pointing at {d}\n", .{current});
    var prev_was_zero = false;
    while (try reader.readUntilDelimiterOrEof(&line_buf, '\n')) |line| {
        const direction = line[0];
        const count = try std.fmt.parseInt(isize, line[1..], 10);
        switch (direction) {
            'L' => current -= count,
            'R' => current += count,
            else => unreachable,
        }
        // std.debug.print("The dial is rotated {c}{}\n", .{direction, count});
        const crossed_zero_times = @divFloor(current, 100);
        current = @mod(current, 100);
        if (current == 0) {
            stopped_at_zero_count += 1;
        }
        var crossed_zero_times_u = @abs(crossed_zero_times);
        if (direction == 'L' and prev_was_zero and crossed_zero_times_u > 0) {
            crossed_zero_times_u -= 1;
        } else if (direction == 'R' and current == 0 and crossed_zero_times_u > 0) {
            crossed_zero_times_u -= 1;
        }
        crossed_zero_count += crossed_zero_times_u;
        prev_was_zero = current == 0;
    }

    std.debug.print("Stopped at zero: {d}\n", .{stopped_at_zero_count});
    std.debug.print("Crossed zero: {d}\n", .{crossed_zero_count});
    const result = stopped_at_zero_count + crossed_zero_count;
    std.debug.print("Result: {d}\n", .{result});
    std.debug.print("Took: {d}ms\n", .{@divFloor(t.read(), std.time.ns_per_ms)});
}
