//! Hello-world snippet for `ProgrammingLanguageName::Fastapi`.
//!
//! Imported from leachim6/hello-world (`f/FastAPI.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/FastAPI.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Python, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Fastapi => "FastAPI",
    versions: [Fastapi => "FastAPI"],
    extensions: [],
}

const FASTAPI: &str = concat!(
    r#"from fastapi import Body, FastAPI

app = FastAPI()

@app.get("/")
async def root():
    return {"message": ""#,
    output_for_hello_world!(),
    r#""}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fastapi,
        extensions: &[],
        categories: &[Framework],
        families: &[Python],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fastapi,
            syntax: FASTAPI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FASTAPI,
                    [
                        (TokenKind::Keyword, r"from"),
                        (TokenKind::Plain, r" fastapi "),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" Body"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r" FastAPI

app ",
                        ),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"FastAPI"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Punctuation, r"@"),
                        (TokenKind::Plain, r"app"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"get"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""/""#),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"async"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"def"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"root"),
                        (TokenKind::Punctuation, r"():"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (TokenKind::String, r#""message""#),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
