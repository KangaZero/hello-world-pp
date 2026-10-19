//! Hello-world snippet for `ProgrammingLanguageName::Byter`.
//!
//! Imported from leachim6/hello-world (`b/Byter.byter` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Byter => "Byter",
    versions: [Byter => "Byter"],
    extensions: [Byter => "byter"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Byter,
        extensions: &[FileExtension::Byter],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Byter,
            syntax: r">>>>>>>>>V00V00V
0#00A00V<V00V0VV
-+>>A00VV<<0$0V>
V0AA<00V>V<<<<V0
>>V0V<<<-<000V<0
0AV0VV<<<0000V<0
0A>V+}>>>>>>}<V-
0A{>>>>-000000>V
00A0000>>>>>>>>A
V0A<<<<<<<<<<<<>
V000000000000000
V000000000000000
V000000000000000
V000000000000000
V000000000000000
>>>>>>>>>>>>V000",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r">>>>>>>>>V00V00V
0#00A00V<V00V0VV
-+>>A00VV<<0$0V>
V0AA<00V>V<<<<V0
>>V0V<<<-<000V<0
0AV0VV<<<0000V<0
0A>V+}>>>>>>}<V-
0A{>>>>-000000>V
00A0000>>>>>>>>A
V0A<<<<<<<<<<<<>
V000000000000000
V000000000000000
V000000000000000
V000000000000000
V000000000000000
>>>>>>>>>>>>V000",
            )],
        }],
    };
