const std = @import("std");

const Token = union(enum) {
    Int: []const u8,
    Ident: []const u8,
    Illegal,
    Eof,
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    If,
    Else,
    Return,
    True,
    False,
    Eq,
    Noteq,

    fn getKeyword(word: []const u8) ?Token {
        const wordMap = std.ComptimeStringMap(Token, .{
            .{ "let", .Let },
            .{ "fn", .Function },
            .{ "if", .If },
            .{ "else", .Else },
            .{ "return", .Return },
            .{ "true", .True },
            .{ "false", .False },
        });

        return wordMap.get(word);
    }
};

pub const Lexer = struct {
    const Self = @This();

    input: []const u8,
    read_position: usize = 0,
    position: usize = 0,
    char: u8 = 0,

    pub fn init(input: []const u8) Self {
        var lexer = Self{
            .input = input,
        };

        lexer.readNext();

        return lexer;
    }

    fn readNext(self: *Self) void {
        if (self.read_position >= self.input.len) {
            self.char = 0;
        } else {
            self.char = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn nextToken(self: *Self) Token {
        self.skipWhitespaces();

        const token: Token = switch (self.char) {
            '{' => .Lbrace,
            '}' => .Rbrace,
            '(' => .Lparen,
            ')' => .Rparen,
            ',' => .Comma,
            ';' => .Semicolon,
            '+' => .Plus,
            '=' => .Assign,
            '*' => .Asterisk,
            '-' => .Minus,
            '/' => .Slash,
            '>' => .Gt,
            '<' => .Lt,
            '0'...'9' => {
                const int = self.readInt();
                return .{ .Ident = int };
            },
            'a'...'z', 'A'...'Z', '_' => {
                const ident = self.readIdent();
                if (Token.getKeyword(ident)) |token| {
                    return token;
                }
                return .{ .Ident = ident };
            },
            0 => .Eof,
            else => .Illegal,
        };

        self.readNext();

        return token;
    }

    fn skipWhitespaces(self: *Self) void {
        while (std.ascii.isWhitespace(self.char)) {
            self.readNext();
        }
    }

    fn readInt(self: *Self) []const u8 {
        const cur_position = self.position;
        if (std.ascii.isAlphanumeric(self.char)) {
            self.readNext();
        }

        return self.input[cur_position..self.position];
    }

    fn readIdent(self: *Self) []const u8 {
        const cur_position = self.position;
        if (std.ascii.isAlphabetic(self.char) or self.char == '_') {
            self.readNext();
        }

        return self.input[cur_position..self.position];
    }
};

test "lexer test 1" {
    var input =
        \\let five = 5;
        \\let ten = 10;
        \\let add = fn(x, y) {
        \\x + y;
        \\};
        \\let result = add(five, ten);
    ;

    var tokens = [_]Token{
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

    var lexer = Lexer.init(input);
    for (tokens) |expectedToken| {
        const token = lexer.nextToken();
        std.debug.print("Expected {} received {}", .{ expectedToken, token });
        try std.testing.expectEqual(expectedToken, token);
    }
}
