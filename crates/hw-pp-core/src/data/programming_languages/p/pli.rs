//! Hello-world snippet for `ProgrammingLanguageName::Pli`.
//!
//! Imported from leachim6/hello-world (`p/PLI.pl1` at `a152253`),
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
    language: Pli => "PLI",
    versions: [Pli => "PLI"],
    extensions: [Pl1 => "pl1"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pli,
        extensions: &[FileExtension::Pl1],
        categories: &[],
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
