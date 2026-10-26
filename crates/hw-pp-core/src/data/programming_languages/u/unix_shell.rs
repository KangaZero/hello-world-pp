//! Hello-world snippet for `ProgrammingLanguageName::UnixShell`.
//!
//! Imported from the Hello World Collection (`files/Unix&nbsp;Shell.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Unix%26nbsp%3BShell.htm>
//! - <http://helloworldcollection.de/#Unix%C2%A0Shell>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: UnixShell => "Unix Shell",
    versions: [UnixShell => "Unix Shell"],
    extensions: [],
}

const UNIX_SHELL: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r" for the Unix shells (sh, ksh, csh, zsh, bash, fish, xonsh, ...)

echo ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::UnixShell,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::UnixShell,
            syntax: UNIX_SHELL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    UNIX_SHELL,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"# ",
                                output_for_hello_world!(),
                                r" for the Unix shells (sh, ksh, csh, zsh, bash, fish, xonsh, ...)"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"echo"),
                        (TokenKind::Plain, concat!(r" ", output_for_hello_world!())),
                    ],
                )
            },
        }],
    };
