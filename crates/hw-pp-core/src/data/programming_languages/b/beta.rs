//! Hello-world snippet for `ProgrammingLanguageName::Beta`.
//!
//! Imported from leachim6/hello-world (`b/Beta.bet` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Beta => "Beta",
    versions: [Beta => "Beta"],
    extensions: [Bet => "bet"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Beta,
        extensions: &[FileExtension::Bet],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Beta,
            syntax: concat!(
                r"ORIGIN '~beta/basiclib/betaenv'
-- program: Descriptor --
(* ",
                output_for_hello_world!(),
                r" in BETA *)
(# do '",
                output_for_hello_world!(),
                r"' -> putLine #)"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"ORIGIN '~beta/basiclib/betaenv'
-- program: Descriptor --
(* ",
                    output_for_hello_world!(),
                    r" in BETA *)
(# do '",
                    output_for_hello_world!(),
                    r"' -> putLine #)"
                ),
            )],
        }],
    };
