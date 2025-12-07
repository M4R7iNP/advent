const std = @import("std");

const MUL = "mul(";
const DO = "do(";
const DONT = "don't(";

const Hei = union(enum) {
    MUL: @TypeOf(MUL),
    DO: @TypeOf(DO),
    DONT: @TypeOf(DONT),
};

const Awd = enum {
    mul,
    do,
    dont,
};

const IndexOfResult = struct {
    index: usize,
    len: usize,
    value: Awd,
};

pub fn indexOfAnyPos(slice: []const u8, start_index: usize) ?IndexOfResult {
    if (start_index >= slice.len) return null;
    for (start_index..slice.len) |i| {
        const c = slice[i..];
        if (std.mem.startsWith(u8, c, MUL)) {
            const result: IndexOfResult = .{ .index = i, .len = MUL.len, .value = Awd.mul };
            return result;
        }
        if (std.mem.startsWith(u8, c, DO)) {
            const result: IndexOfResult = .{ .index = i, .len = DO.len, .value = Awd.do };
            return result;
        }
        if (std.mem.startsWith(u8, c, DONT)) {
            const result: IndexOfResult = .{ .index = i, .len = DONT.len, .value = Awd.dont };
            return result;
        }
    }
    return null;
}

var NEEDLES = [_][]const u8{ "mul(", "don't(", "do(" };

pub fn main() !void {
    std.debug.print("BEGIN\n", .{});
    var t = try std.time.Timer.start();
    const stdin = std.io.getStdIn();
    const allocator = std.heap.page_allocator;
    const reader = stdin.reader();
    var sum: usize = 0;
    var count: usize = 0;

    var is_enabled = true;

    while (try reader.readUntilDelimiterOrEofAlloc(allocator, '\n', 4096)) |line| {
        var cursor: usize = 0;
        // while (std.mem.indexOfAnyPos(u8, line, cursor, &NEEDLES)) |new_cursor| {
        while (indexOfAnyPos(line, cursor)) |result| {
            cursor = result.index;
            if (result.value == Awd.dont) {
                is_enabled = false;
                cursor += 1;
                continue;
            } else if (result.value == Awd.do) {
                is_enabled = true;
                cursor += 1;
                continue;
            } else if (!is_enabled) {
                cursor += 1;
                continue;
            }
            cursor += "mul(".len;
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
