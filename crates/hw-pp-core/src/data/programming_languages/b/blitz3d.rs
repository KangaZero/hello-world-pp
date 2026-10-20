//! Hello-world snippet for `ProgrammingLanguageName::Blitz3d`.
//!
//! Imported from leachim6/hello-world (`b/Blitz3D.bb` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Blitz3D.bb>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Blitz3d => "Blitz3D",
    versions: [Blitz3d => "Blitz3D"],
    extensions: [Bb => "bb"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blitz3d,
        extensions: &[FileExtension::Bb],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blitz3d,
            syntax: concat!(
                r";Blitz3D ",
                output_for_hello_world!(),
                r#" demo by MANIAK_dobrii
Print ""#,
                output_for_hello_world!(),
                r#""
WaitKey
End"#
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    concat!(
                        r"Blitz3D ",
                        output_for_hello_world!(),
                        r" demo by MANIAK_dobrii
"
                    ),
                ),
                (TokenKind::Keyword, r"Print"),
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
                (TokenKind::Keyword, r"WaitKey"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"End"),
            ],
        }],
    };
