//! Hello-world snippet for `ProgrammingLanguageName::Koa`.
//!
//! Imported from leachim6/hello-world (`k/Koa.js` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Koa.js>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Koa => "Koa",
    versions: [Koa => "Koa"],
    extensions: [],
}

const KOA: &str = concat!(
    r"const Koa = require('koa');
const app = new Koa();

app.use(async ctx => {
  ctx.body = '",
    output_for_hello_world!(),
    r"';
});

app.listen(8080);"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Koa,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Koa,
            syntax: KOA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KOA,
                    [
                        (TokenKind::Type, r"const"),
                        (TokenKind::Plain, r" Koa "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"require"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r"'koa'"),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Type, r"const"),
                        (TokenKind::Plain, r" app "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"new"),
                        (TokenKind::Plain, r" Koa"),
                        (TokenKind::Punctuation, r"();"),
                        (
                            TokenKind::Plain,
                            r"

app",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"use"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Type, r"async"),
                        (TokenKind::Plain, r" ctx "),
                        (TokenKind::Keyword, r"=>"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  ctx",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"body "),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"});"),
                        (
                            TokenKind::Plain,
                            r"

app",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"listen"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Number, r"8080"),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };
