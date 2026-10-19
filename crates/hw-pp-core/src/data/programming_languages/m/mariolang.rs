//! Hello-world snippet for `ProgrammingLanguageName::Mariolang`.
//!
//! Imported from leachim6/hello-world (`m/MarioLANG.mlg` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Mariolang => "MarioLANG",
    versions: [Mariolang => "MarioLANG"],
    extensions: [Mlg => "mlg"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mariolang,
        extensions: &[FileExtension::Mlg],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mariolang,
            syntax: r#"+>+>)+)+)+++)++++((((-[!)>->.
+"+"===================#+".")
+++!((+++++++++)++++++)<.---+
++=#===================")---.
++((.-(.)).+++..+++++++.<---
 !+======================---
=#>++++++++++++++.).+++.-!>!
  =======================#=#"#,
            tokenized_syntax: &[(
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
        }],
    };
