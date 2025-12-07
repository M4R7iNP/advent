const std = @import("std");

const State = enum {
    CardNumber,
    WinningNumbers,
    VerifyNumbers,
};

fn digitsToNumbers(digits_buf: [16]u8, len: usize) usize {
    var number: usize = 0;
    for (0..len) |num_idx| {
        number += digits_buf[num_idx] * std.math.pow(usize, 10, len - num_idx - 1);
    }
    return number;
}

pub fn main() !void {
    std.debug.print("BEGIN\n", .{});
    var t = try std.time.Timer.start();
    var file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();
    var bufReader = std.io.bufferedReader(file.reader());
    var reader = bufReader.reader();
    var line_buf: [256]u8 = undefined;
    var total_points: usize = 0;
    var card_counts: [16]usize = .{};
    var total_number_of_cards: usize = 0;
    @memset(&card_counts, 1);

    while (try reader.readUntilDelimiterOrEof(&line_buf, '\n')) |line| {
        var card_count = card_counts[0];
        total_number_of_cards += card_count;
        std.mem.copyForwards(usize, &card_counts, card_counts[1..]);
        card_counts[card_counts.len - 1] = 1;

        var state: State = State.CardNumber;
        var winning_numbers: [10]usize = undefined;
        var winning_numbers_len: usize = 0;

        var digits_buf: [16]u8 = .{};
        var digits_len: usize = 0;
        var card_points: usize = 0;
        var correct_numbers: usize = 0;

        for (0..(line.len + 1)) |char_idx| {
            var char: u8 = if (char_idx < line.len) line[char_idx] else ' ';

            if (char == ':') {
                state = State.WinningNumbers;
                digits_buf = .{};
                digits_len = 0;
                continue;
            } else if (char == '|') {
                state = State.VerifyNumbers;
                digits_buf = .{};
                digits_len = 0;
                continue;
            }

            if (state == State.WinningNumbers) {
                if (std.ascii.isDigit(char)) {
                    digits_buf[digits_len] = char - '0';
                    digits_len += 1;
                } else if ((char == ' ') and (digits_len > 0)) {
                    var number = digitsToNumbers(digits_buf, digits_len);
                    winning_numbers[winning_numbers_len] = number;
                    winning_numbers_len += 1;
                    digits_buf = .{};
                    digits_len = 0;
                }
            } else if (state == State.VerifyNumbers) {
                if (std.ascii.isDigit(char)) {
                    digits_buf[digits_len] = char - '0';
                    digits_len += 1;
                } else if (char == ' ') {
                    var number = digitsToNumbers(digits_buf, digits_len);
                    var is_winning = for (winning_numbers) |n| {
                        if (n == number) {
                            break true;
                        }
                    } else false;

                    if (is_winning) {
                        correct_numbers += 1;
                        if (card_points == 0) {
                            card_points = 1;
                        } else {
                            card_points *= 2;
                        }
                    }

                    digits_buf = .{};
                    digits_len = 0;
                }
            }
        }

        for (0..correct_numbers) |n| {
            card_counts[n] += card_count;
        }

        total_points += card_points;
        correct_numbers = 0;
    }

    std.debug.print("Took: {d}ms\n", .{@divFloor(t.read(), std.time.ns_per_ms)});
    std.debug.print("Total points: {d}\n", .{total_points});
    std.debug.print("Total number of cards: {d}\n", .{total_number_of_cards});
}
