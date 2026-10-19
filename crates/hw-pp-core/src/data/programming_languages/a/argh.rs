//! Hello-world snippet for `ProgrammingLanguageName::Argh`.
//!
//! Imported from leachim6/hello-world (`a/Argh.agh` at `a152253`),
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
    language: Argh => "Argh",
    versions: [Argh => "Argh"],
    extensions: [Agh => "agh"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Argh,
        extensions: &[FileExtension::Agh],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Argh,
            syntax: concat!(
                r"lpppppppppppsrfj
 ",
                output_for_hello_world!(),
                r"*  j
             qPh"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"lpppppppppppsrfj
 ",
                    output_for_hello_world!(),
                    r"*  j
             qPh"
                ),
            )],
        }],
    };
