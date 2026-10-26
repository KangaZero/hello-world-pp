//! Hello-world snippet for `ProgrammingLanguageName::Mariolang`.
//!
//! Imported from leachim6/hello-world (`m/MarioLANG.mlg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/MarioLANG.mlg>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mariolang => "MarioLANG",
    versions: [Mariolang => "MarioLANG"],
    extensions: [Mlg => "mlg"],
}

const MARIOLANG: &str = r#"+>+>)+)+)+++)++++((((-[!)>->.
+"+"===================#+".")
+++!((+++++++++)++++++)<.---+
++=#===================")---.
++((.-(.)).+++..+++++++.<---
 !+======================---
=#>++++++++++++++.).+++.-!>!
  =======================#=#"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mariolang,
        extensions: &[FileExtension::Mlg],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mariolang,
            syntax: MARIOLANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MARIOLANG,
                    [(
                        TokenKind::Plain,
                        r#"+>+>)+)+)+++)++++((((-[!)>->.
+"+"===================#+".")
+++!((+++++++++)++++++)<.---+
++=#===================")---.
++((.-(.)).+++..+++++++.<---
 !+======================---
=#>++++++++++++++.).+++.-!>!
  =======================#=#"#,
                    )],
                )
            },
        }],
    };
