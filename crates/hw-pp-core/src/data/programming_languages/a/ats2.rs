//! Hello-world snippets for `ProgrammingLanguageName::Ats2`.
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
//! - <https://en.wikipedia.org/wiki/ATS_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Concurrent, Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Ats2 => "ATS2",
    versions: [Ats2 => "ATS2", Ats => "ATS"],
    extensions: [Dats => "dats"],
}

const ATS2: &str = concat!(
    r#"implement main0 () = println! ""#,
    output_for_hello_world!(),
    r#"""#
);
const ATS: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in ATS

implement main () = begin
  print (""#,
    output_for_hello_world!(),
    r#""); print_newline ()
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ats2,
        extensions: &[FileExtension::Dats],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Imperative, ObjectOriented, Functional, Concurrent],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Ats2,
                syntax: ATS2,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        ATS2,
                        [
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
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Ats,
                syntax: ATS,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        ATS,
                        [
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
                    )
                },
            },
        ],
    };
