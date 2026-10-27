//! Hello-world snippet for `ProgrammingLanguageName::LtSpanClassEqSpoilerGtFuckLtSlashSpanGt`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Fuck>
//! - <https://esolangs.org/wiki/SpoilerFuck>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: LtSpanClassEqSpoilerGtFuckLtSlashSpanGt => "<span class=\"spoiler\">Fuck</span>",
    versions: [LtSpanClassEqSpoilerGtFuckLtSlashSpanGt => "<span class=\"spoiler\">Fuck</span>"],
    extensions: [],
}

const LT_SPAN_CLASS_EQ_SPOILER_GT_FUCK_LT_SLASH_SPAN_GT: &str = r#" ++++++++<span class="spoiler">>++++<span class="spoiler">>++>+++>+++>+<<<<-</span>>+>+>->>+<span class="spoiler"><</span><-</span>>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::LtSpanClassEqSpoilerGtFuckLtSlashSpanGt,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::LtSpanClassEqSpoilerGtFuckLtSlashSpanGt,
            syntax: LT_SPAN_CLASS_EQ_SPOILER_GT_FUCK_LT_SLASH_SPAN_GT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LT_SPAN_CLASS_EQ_SPOILER_GT_FUCK_LT_SLASH_SPAN_GT,
                    [(
                        TokenKind::Plain,
                        r#" ++++++++<span class="spoiler">>++++<span class="spoiler">>++>+++>+++>+<<<<-</span>>+>+>->>+<span class="spoiler"><</span><-</span>>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."#,
                    )],
                )
            },
        }],
    };
