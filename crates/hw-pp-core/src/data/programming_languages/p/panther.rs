//! Hello-world snippet for `ProgrammingLanguageName::Panther`.
//!
//! Imported from leachim6/hello-world (`p/Panther.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Panther.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Panther => "Panther",
    versions: [Panther => "Panther"],
    extensions: [],
}

const PANTHER: &str = concat!(
    r"import sys
from panther import Panther
from panther.app import API


@API()
def hello_world():
    return '",
    output_for_hello_world!(),
    r"'


app = Panther(__name__, configs=sys.modules[__name__], urls={'/': hello_world})"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Panther,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Panther,
            syntax: PANTHER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PANTHER,
                    [
                        (TokenKind::Keyword, r"import"),
                        (
                            TokenKind::Plain,
                            r" sys
",
                        ),
                        (TokenKind::Keyword, r"from"),
                        (TokenKind::Plain, r" panther "),
                        (TokenKind::Keyword, r"import"),
                        (
                            TokenKind::Plain,
                            r" Panther
",
                        ),
                        (TokenKind::Keyword, r"from"),
                        (TokenKind::Plain, r" panther"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"app "),
                        (TokenKind::Keyword, r"import"),
                        (
                            TokenKind::Plain,
                            r" API


",
                        ),
                        (TokenKind::Punctuation, r"@"),
                        (TokenKind::Plain, r"API"),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"def"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"hello_world"),
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
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (
                            TokenKind::Plain,
                            r"


app ",
                        ),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"Panther"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"__name__"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" configs"),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Plain, r"sys"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"modules"),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Plain, r"__name__"),
                        (TokenKind::Punctuation, r"],"),
                        (TokenKind::Plain, r" urls"),
                        (TokenKind::Keyword, r"="),
                        (TokenKind::Punctuation, r"{"),
                        (TokenKind::String, r"'/'"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" hello_world"),
                        (TokenKind::Punctuation, r"})"),
                    ],
                )
            },
        }],
    };
