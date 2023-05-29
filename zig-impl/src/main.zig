const std = @import("std");
const lx = @import("lexer/lexer.zig");

test {
    _ = @import("lexer/lexer.zig");
}

pub fn main() !void {
    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Run `zig build test` to run the tests.\n", .{});

    var input =
        \\let five = 5;
        \\let ten = 10;
        \\let add = fn(x, y) {
        \\x + y;
        \\};
        \\let result = add(five, ten);
    ;

    var tokens = [_]lx.Token{
        .Let,
        .{ .Ident = "five" },
        .Assign,
        .{ .Int = "5" },
        .Semicolon,
        .Let,
        .{ .Ident = "ten" },
        .Assign,
        .{ .Int = "10" },
        .Semicolon,
        .Let,
        .{ .Ident = "add" },
        .Assign,
        .Function,
        .Lparen,
        .{ .Ident = "x" },
        .Comma,
        .{ .Ident = "y" },
        .Rparen,
        .Lbrace,
        .{ .Ident = "x" },
        .Plus,
        .{ .Ident = "y" },
        .Semicolon,
        .Rbrace,
        .Semicolon,
        .Let,
        .{ .Ident = "result" },
        .Assign,
        .{ .Ident = "add" },
        .Lparen,
        .{ .Ident = "five" },
        .Comma,
        .{ .Ident = "ten" },
        .Rparen,
        .Semicolon,
        .Eof,
    };

    var lexer = lx.Lexer.init(input);
    for (tokens) |expected| {
        const actual = lexer.nextToken();

        // var a = tk == token;
        const Tag = std.meta.Tag(@TypeOf(expected));

        const expectedTag = @as(Tag, expected);
        _ = expectedTag;
        const actualTag = @as(Tag, actual);
        _ = actualTag;

        // std.debug.print("expectedTag: {} actualTag {}", .{ expectedTag, actualTag });

        // // we only reach this loop if the tags are equal
        // inline for (std.meta.fields(@TypeOf(actual))) |fld| {
        //     if (std.mem.eql(u8, fld.name, @tagName(actualTag))) {
        //         std.debug.print("expected: {} actual {}", .{ @field(expected, fld.name), @field(actual, fld.name) });
        //     }
        // }
    }

    try bw.flush(); // don't forget to flush!
}
