//! Hello-world snippet for `ProgrammingLanguageName::Pli`.
//!
//! Imported from leachim6/hello-world (`p/PLI.pl1` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PLI.pl1>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pli => "PLI",
    versions: [Pli => "PLI"],
    extensions: [Pl1 => "pl1"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pli,
        extensions: &[FileExtension::Pl1],
        categories: &[Compiled, Historical],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pli,
            syntax: concat!(
                r"Hello2: proc options(main);
     put list ('",
                output_for_hello_world!(),
                r"');
end Hello2;"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"Hello2: proc options(main);
     put list ('",
                    output_for_hello_world!(),
                    r"');
end Hello2;"
                ),
            )],
        }],
    };
