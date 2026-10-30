//! Hello-world snippet for `ProgrammingLanguageName::Django`.
//!
//! Imported from leachim6/hello-world (`d/Django.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Django.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Python, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Django => "Django",
    versions: [Django => "Django"],
    extensions: [],
}

const DJANGO: &str = concat!(
    r#"from django.http import HttpResponse

def index(request):
    return HttpResponse(""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Django,
        extensions: &[],
        categories: &[Framework],
        families: &[Python],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Django,
            syntax: DJANGO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DJANGO,
                    [
                        (TokenKind::Keyword, r"from"),
                        (TokenKind::Plain, r" django"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"http "),
                        (TokenKind::Keyword, r"import"),
                        (
                            TokenKind::Plain,
                            r" HttpResponse

",
                        ),
                        (TokenKind::Keyword, r"def"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"index"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"request"),
                        (TokenKind::Punctuation, r"):"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"HttpResponse"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
