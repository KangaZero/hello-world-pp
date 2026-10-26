//! Hello-world snippet for `ProgrammingLanguageName::Gluon`.
//!
//! Imported from leachim6/hello-world (`g/Gluon.glu` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Gluon.glu>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gluon => "Gluon",
    versions: [Gluon => "Gluon"],
    extensions: [Glu => "glu"],
}

const GLUON: &str = concat!(
    r#"let io = import! std.io
io.print ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gluon,
        extensions: &[FileExtension::Glu],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gluon,
            syntax: GLUON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GLUON,
                    [
                        (TokenKind::Keyword, r"let"),
                        (TokenKind::Plain, r" io = "),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r"! std"),
                        (TokenKind::Punctuation, r"."),
                        (
                            TokenKind::Plain,
                            r"io
io",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
