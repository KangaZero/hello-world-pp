//! Hello-world snippet for `ProgrammingLanguageName::PureData`.
//!
//! Imported from leachim6/hello-world (`p/Pure Data.pd` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pure%20Data.pd>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Music,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: PureData => "Pure Data",
    versions: [PureData => "Pure Data"],
    extensions: [Pd => "pd"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PureData,
        extensions: &[FileExtension::Pd],
        categories: &[Music],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PureData,
            syntax: concat!(
                r"#N canvas 1029 457 450 300 10;
#X obj 127 132 print;
#X msg 127 86 ",
                output_for_hello_world!(),
                r";
#X connect 1 0 0 0;"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"#N canvas 1029 457 450 300 10;
#X obj 127 132 print;
#X msg 127 86 ",
                    output_for_hello_world!(),
                    r";
#X connect 1 0 0 0;"
                ),
            )],
        }],
    };
