//! Hello-world snippet for `ProgrammingLanguageName::Ats2`.
//!
//! Imported from leachim6/hello-world (`a/ATS2.dats` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/ATS.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ATS2.dats>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ATS.htm>
//! - <http://helloworldcollection.de/#ATS>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ats2 => "ATS2",
    versions: [Ats2 => "ATS2", Ats => "ATS"],
    extensions: [Dats => "dats"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ats2,
        extensions: &[FileExtension::Dats],
        categories: &[Compiled],
        families: &[],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Ats2,
                syntax: concat!(
                    r#"implement main0 () = println! ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
                tokenized_syntax: &[
                    (TokenKind::Plain, r"implement "),
                    (TokenKind::Function, r"main0"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"()"),
                    (TokenKind::Plain, r" = "),
                    (TokenKind::Function, r"println"),
                    (TokenKind::Plain, r"! "),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Ats,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in ATS

implement main () = begin
  print (""#,
                    output_for_hello_world!(),
                    r#""); print_newline ()
end"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(r"// ", output_for_hello_world!(), r" in ATS"),
                    ),
                    (
                        TokenKind::Plain,
                        r"

implement ",
                    ),
                    (TokenKind::Function, r"main"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"()"),
                    (
                        TokenKind::Plain,
                        r" = begin
  ",
                    ),
                    (TokenKind::Function, r"print"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"("),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r");"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"print_newline"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"()"),
                    (
                        TokenKind::Plain,
                        r"
end",
                    ),
                ],
            },
        ],
    };
