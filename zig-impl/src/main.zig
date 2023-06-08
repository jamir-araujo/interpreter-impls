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

    const input = "let tests_var = 10;";
    var lex = lx.Lexer.init(input);

    var tokens = [_]lx.Token{
        .Let,
        .{ .Ident = "tests_var" },
        .Assign,
        .{ .Int = "10" },
        .Semicolon,
    };

    for (tokens) |token| {
        const tok = lex.nextToken();

        var name = @tagName(tok);
        var t_name = @tagName(token);

        std.debug.print("expected: {s}, actual: {s}\n", .{ t_name, name });

        switch (token) {
            .Ident => |id| std.debug.print("expected: {s}, actual: {s},\n", .{ "tests_var", id }),
            .Int => |integer| std.debug.print("expected: {s}, actual: {s}", .{ "10", integer }),
            else => std.debug.print("expected: {}, actual: {},\n", .{ token, tok }),
        }

        std.debug.print("\n\n\nnew test\n", .{});

        // std.debug.print(t_name, name);

        // try std.testing.expectEqual(t_name, name);
        // try std.testing.expectEqual(token, tok);
    }

    try bw.flush(); // don't forget to flush!
}
