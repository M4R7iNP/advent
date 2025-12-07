const std = @import("std");

pub fn main() !void {
    std.debug.print("BEGIN\n", .{});
    var t = try std.time.Timer.start();
    const stdin = std.io.getStdIn();
    const allocator = std.heap.page_allocator;
    const reader = stdin.reader();
    var sum: usize = 0;
    var count: usize = 0;

    while (try reader.readUntilDelimiterOrEofAlloc(allocator, '\n', 4096)) |line| {
        var cursor: usize = 0;
        while (std.mem.indexOfPosLinear(u8, line, cursor, "mul(")) |awd| {
            cursor = awd + "mul(".len;
            const end = std.mem.indexOfPosLinear(u8, line, cursor, ")") orelse continue;
            const args_str = line[cursor..end];
            var parts = std.mem.split(u8, args_str, ",");
            const first_str = parts.next() orelse unreachable;
            const second_str = parts.next() orelse continue;
            const first_num = std.fmt.parseInt(usize, first_str, 10) catch continue;
            const second_num = std.fmt.parseInt(usize, second_str, 10) catch continue;
            if (parts.next() != null) {
                continue;
            }
            sum += first_num * second_num;
            count += 1;
            // std.debug.print("{d} * {d} = {d}\n", .{ first_num, second_num, first_num * second_num });
            cursor = end;
        }
    }

    std.debug.print("Result: {d}\n", .{sum});
    std.debug.print("Count: {d}\n", .{count});
    std.debug.print("Took: {d}ms\n", .{@divFloor(t.read(), std.time.ns_per_ms)});
}
