//! Hello-world snippets for `ProgrammingLanguageName::Befunge`.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Befunge>
//! - <https://esolangs.org/wiki/Funge-98>
//! - <https://catseye.tc/view/Befunge-93/doc/Befunge-93.markdown>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Befunge => "Befunge",
    versions: [Befunge93 => "Befunge-93", Funge98 => "Funge-98"],
    extensions: [Bf => "bf", Be => "be", B93 => "b93", B98 => "b98", Befunge => "befunge"],
}

const BEFUNGE93: &str = r#""!dlroW ,olleH">:#,_@"#;
const FUNGE98: &str = concat!("<v\"", output_for_hello_world!(), "\"\n >:v\n ^,_@");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Befunge,
        extensions: &[FileExtension::Bf],
        categories: &[Esolang],
        families: &[],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Befunge93,
                syntax: BEFUNGE93,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        BEFUNGE93,
                        [
                            (TokenKind::String, "\"!dlroW ,olleH\""),
                            (TokenKind::Keyword, ">"),
                            (TokenKind::Function, ":"),
                            (TokenKind::Keyword, "#"),
                            (TokenKind::Function, ","),
                            (TokenKind::Keyword, "_@"),
                        ],
                    )
                },
            },
            // `<` wraps the IP round to read the string backwards, so it needs no reversing.
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Funge98,
                syntax: FUNGE98,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        FUNGE98,
                        [
                            (TokenKind::Keyword, "<v"),
                            (
                                TokenKind::String,
                                concat!("\"", output_for_hello_world!(), "\""),
                            ),
                            (TokenKind::Plain, "\n "),
                            (TokenKind::Keyword, ">"),
                            (TokenKind::Function, ":"),
                            (TokenKind::Keyword, "v"),
                            (TokenKind::Plain, "\n "),
                            (TokenKind::Keyword, "^"),
                            (TokenKind::Function, ","),
                            (TokenKind::Keyword, "_@"),
                        ],
                    )
                },
            },
        ],
    };
