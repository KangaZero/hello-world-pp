//! Hello-world snippets for `ProgrammingLanguageName::Autoit`.
//!
//! Imported from leachim6/hello-world (`a/AutoIt.au3` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/AutoIT3.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/AutoIt.au3>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/AutoIT3.txt>
//! - <http://helloworldcollection.de/#AutoIT3>
//! - <https://www.wikidata.org/wiki/Q784845> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, Procedural, Reflective,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Autoit => "AutoIt",
    versions: [Autoit => "AutoIt", Autoit3 => "AutoIT3"],
    extensions: [Au3 => "au3"],
}

const AUTOIT: &str = concat!(
    r#"MsgBox(0, "Message Box", ""#,
    output_for_hello_world!(),
    r#"")"#
);
const AUTOIT3: &str = concat!(
    r";",
    output_for_hello_world!(),
    r#" for AutoIT3  http://www.autoitscript.com

msgbox(0,""#,
    output_for_hello_world!(),
    r#"",0)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Autoit,
        extensions: &[FileExtension::Au3],
        categories: &[Scripting],
        families: &[Basic],
        paradigms: &[
            Imperative,
            Procedural,
            ObjectOriented,
            Functional,
            Reflective,
        ],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Autoit,
                syntax: AUTOIT,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        AUTOIT,
                        [
                            (TokenKind::Keyword, r"MsgBox"),
                            (TokenKind::Punctuation, r"("),
                            (TokenKind::Number, r"0"),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (TokenKind::String, r#""Message Box""#),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Plain, r" "),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#"""#),
                            ),
                            (TokenKind::Punctuation, r")"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Autoit3,
                syntax: AUTOIT3,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        AUTOIT3,
                        [
                            (
                                TokenKind::Comment,
                                concat!(
                                    r";",
                                    output_for_hello_world!(),
                                    r" for AutoIT3  http://www.autoitscript.com"
                                ),
                            ),
                            (
                                TokenKind::Plain,
                                r"

",
                            ),
                            (TokenKind::Keyword, r"msgbox"),
                            (TokenKind::Punctuation, r"("),
                            (TokenKind::Number, r"0"),
                            (TokenKind::Punctuation, r","),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#"""#),
                            ),
                            (TokenKind::Punctuation, r","),
                            (TokenKind::Number, r"0"),
                            (TokenKind::Punctuation, r")"),
                        ],
                    )
                },
            },
        ],
    };
