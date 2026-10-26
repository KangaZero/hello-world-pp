//! Hello-world snippet for `ProgrammingLanguageName::Beta`.
//!
//! Imported from leachim6/hello-world (`b/Beta.bet` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Beta.bet>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Beta => "Beta",
    versions: [Beta => "Beta"],
    extensions: [Bet => "bet"],
}

const BETA: &str = concat!(
    r"ORIGIN '~beta/basiclib/betaenv'
-- program: Descriptor --
(* ",
    output_for_hello_world!(),
    r" in BETA *)
(# do '",
    output_for_hello_world!(),
    r"' -> putLine #)"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Beta,
        extensions: &[FileExtension::Bet],
        categories: &[Compiled, Historical],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Beta,
            syntax: BETA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BETA,
                    [(
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
                )
            },
        }],
    };
