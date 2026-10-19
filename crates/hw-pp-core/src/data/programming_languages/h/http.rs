//! Hello-world snippet for `ProgrammingLanguageName::Http`.
//!
//! Imported from leachim6/hello-world (`h/HTTP.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/HTTP.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Http => "HTTP",
    versions: [Http => "HTTP"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Http,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Http,
            syntax: concat!(
                r"import http.server
import socketserver
from http import HTTPStatus


class Handler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        self.send_response(HTTPStatus.OK)
        self.end_headers()
        self.wfile.write(b'",
                output_for_hello_world!(),
                r"')


httpd = socketserver.TCPServer(('', 8000), Handler)
httpd.serve_forever()"
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" http"),
                (TokenKind::Punctuation, r"."),
                (
                    TokenKind::Plain,
                    r"server
",
                ),
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" socketserver
",
                ),
                (TokenKind::Keyword, r"from"),
                (TokenKind::Plain, r" http "),
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" HTTPStatus


",
                ),
                (TokenKind::Keyword, r"class"),
                (TokenKind::Plain, r" Handler"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"http"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"server"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"SimpleHTTPRequestHandler"),
                (TokenKind::Punctuation, r"):"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"def"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"do_GET"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"self"),
                (TokenKind::Punctuation, r"):"),
                (
                    TokenKind::Plain,
                    r"
        self",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"send_response"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"HTTPStatus"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"OK"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
        self",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"end_headers"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"
        self",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"wfile"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"write"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Type, r"b"),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"


httpd ",
                ),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" socketserver"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"TCPServer"),
                (TokenKind::Punctuation, r"(("),
                (TokenKind::String, r"''"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" "),
                (TokenKind::Number, r"8000"),
                (TokenKind::Punctuation, r"),"),
                (TokenKind::Plain, r" Handler"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
httpd",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"serve_forever"),
                (TokenKind::Punctuation, r"()"),
            ],
        }],
    };
