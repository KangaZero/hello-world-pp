//! Hello-world snippet for `ProgrammingLanguageName::PowerFx`.
//!
//! Imported from leachim6/hello-world (`p/Power Fx.pfx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Power%20Fx.pfx>
//! - <https://www.wikidata.org/wiki/Q105743047> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Declarative, Functional, Imperative,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: PowerFx => "Power Fx",
    versions: [PowerFx => "Power Fx"],
    extensions: [Pfx => "pfx"],
}

const POWER_FX: &str = concat!(r#"Text(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PowerFx,
        extensions: &[FileExtension::Pfx],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Imperative, Functional, Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PowerFx,
            syntax: POWER_FX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    POWER_FX,
                    [(
                        TokenKind::Plain,
                        concat!(r#"Text(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
