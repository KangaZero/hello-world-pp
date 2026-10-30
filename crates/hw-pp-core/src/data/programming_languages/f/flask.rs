//! Hello-world snippet for `ProgrammingLanguageName::Flask`.
//!
//! Imported from leachim6/hello-world (`f/Flask.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Flask.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Python, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Flask => "Flask",
    versions: [Flask => "Flask"],
    extensions: [],
}

const FLASK: &str = concat!(
    r#"from flask import Flask
app = Flask(__name__)

@app.route("/")
def hello():
    return ""#,
    output_for_hello_world!(),
    r#""

if __name__ == "__main__":
    app.run()"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Flask,
        extensions: &[],
        categories: &[Framework],
        families: &[Python],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Flask,
            syntax: FLASK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FLASK,
                    [
                        (TokenKind::Keyword, r"from"),
                        (TokenKind::Plain, r" flask "),
                        (TokenKind::Keyword, r"import"),
                        (
                            TokenKind::Plain,
                            r" Flask
app ",
                        ),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"Flask"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"__name__"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Punctuation, r"@"),
                        (TokenKind::Plain, r"app"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"route"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""/""#),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"def"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"hello"),
                        (TokenKind::Punctuation, r"():"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"if"),
                        (TokenKind::Plain, r" __name__ "),
                        (TokenKind::Keyword, r"=="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""__main__""#),
                        (TokenKind::Punctuation, r":"),
                        (
                            TokenKind::Plain,
                            r"
    app",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"run"),
                        (TokenKind::Punctuation, r"()"),
                    ],
                )
            },
        }],
    };
