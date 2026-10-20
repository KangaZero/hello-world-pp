//! Hello-world snippet for `ProgrammingLanguageName::LtSpanClassEqSpoilerGtFuckLtSlashSpanGt`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#%3Cspan_class%3D%22spoiler%22%3EFuck%3C%2Fspan%3E>
//! - <https://esolangs.org/wiki/SpoilerFuck>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: LtSpanClassEqSpoilerGtFuckLtSlashSpanGt => "<span class=\"spoiler\">Fuck</span>",
    versions: [LtSpanClassEqSpoilerGtFuckLtSlashSpanGt => "<span class=\"spoiler\">Fuck</span>"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LtSpanClassEqSpoilerGtFuckLtSlashSpanGt,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LtSpanClassEqSpoilerGtFuckLtSlashSpanGt,
            syntax: r#" ++++++++<span class="spoiler">>++++<span class="spoiler">>++>+++>+++>+<<<<-</span>>+>+>->>+<span class="spoiler"><</span><-</span>>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."#,
            tokenized_syntax: &[(
                TokenKind::Plain,
                r#" ++++++++<span class="spoiler">>++++<span class="spoiler">>++>+++>+++>+<<<<-</span>>+>+>->>+<span class="spoiler"><</span><-</span>>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."#,
            )],
        }],
    };
