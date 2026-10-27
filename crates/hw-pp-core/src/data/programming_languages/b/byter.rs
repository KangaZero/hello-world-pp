//! Hello-world snippet for `ProgrammingLanguageName::Byter`.
//!
//! Imported from leachim6/hello-world (`b/Byter.byter` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Byter.byter>
//! - <https://esolangs.org/wiki/Byter> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Reflective, TwoDimensional};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Byter => "Byter",
    versions: [Byter => "Byter"],
    extensions: [Byter => "byter"],
}

const BYTER: &str = r">>>>>>>>>V00V00V
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
>>>>>>>>>>>>V000";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Byter,
        extensions: &[FileExtension::Byter],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Reflective, TwoDimensional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Byter,
            syntax: BYTER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BYTER,
                    [(
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
                )
            },
        }],
    };
