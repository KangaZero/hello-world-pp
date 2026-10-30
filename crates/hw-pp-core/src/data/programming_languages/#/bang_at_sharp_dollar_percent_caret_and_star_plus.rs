//! Hello-world snippet for `ProgrammingLanguageName::BangAtSharpDollarPercentCaretAndStarPlus`.
//!
//! Imported from leachim6/hello-world (`#/!@#$%^&∗()_+` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%21%40%23%24%25%5E%26%E2%88%97%28%29_%2B>
//! - <https://esolangs.org/wiki/%21%40%24%25%5E%26%2A%28%29%2B> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: BangAtSharpDollarPercentCaretAndStarPlus => "!@#$%^&∗()_+",
    versions: [BangAtSharpDollarPercentCaretAndStarPlus => "!@#$%^&∗()_+"],
    extensions: [],
}

const BANG_AT_SHARP_DOLLAR_PERCENT_CARET_AND_STAR_PLUS: &str = r"dlroW olleH(@)";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BangAtSharpDollarPercentCaretAndStarPlus,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BangAtSharpDollarPercentCaretAndStarPlus,
            syntax: BANG_AT_SHARP_DOLLAR_PERCENT_CARET_AND_STAR_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BANG_AT_SHARP_DOLLAR_PERCENT_CARET_AND_STAR_PLUS,
                    [(TokenKind::Plain, r"dlroW olleH(@)")],
                )
            },
        }],
    };
