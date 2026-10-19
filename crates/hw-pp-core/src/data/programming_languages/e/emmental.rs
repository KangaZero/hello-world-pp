//! Hello-world snippet for `ProgrammingLanguageName::Emmental`.
//!
//! Imported from leachim6/hello-world (`e/Emmental.emmental` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Emmental.emmental>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Emmental => "Emmental",
    versions: [Emmental => "Emmental"],
    extensions: [Emmental => "emmental"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Emmental,
        extensions: &[FileExtension::Emmental],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Emmental,
            syntax: r";#58#126#63#36!;#46#36#!;#0#1!;#0#2!;#0#3!;#0#4!;#0#5!;#0#6!;#0#7!#0#33#100#108#114#111#119#32#44#111#108#108#101#72$",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r";#58#126#63#36!;#46#36#!;#0#1!;#0#2!;#0#3!;#0#4!;#0#5!;#0#6!;#0#7!#0#33#100#108#114#111#119#32#44#111#108#108#101#72$",
            )],
        }],
    };
