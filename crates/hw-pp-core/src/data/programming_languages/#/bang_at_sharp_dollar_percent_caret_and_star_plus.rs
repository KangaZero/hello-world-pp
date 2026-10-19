//! Hello-world snippet for `ProgrammingLanguageName::BangAtSharpDollarPercentCaretAndStarPlus`.
//!
//! Imported from leachim6/hello-world (`#/!@#$%^&∗()_+` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%21%40%23%24%25%5E%26%E2%88%97%28%29_%2B>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: BangAtSharpDollarPercentCaretAndStarPlus => "!@#$%^&∗()_+",
    versions: [BangAtSharpDollarPercentCaretAndStarPlus => "!@#$%^&∗()_+"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BangAtSharpDollarPercentCaretAndStarPlus,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BangAtSharpDollarPercentCaretAndStarPlus,
            syntax: r"dlroW olleH(@)",
            tokenized_syntax: &[(TokenKind::Plain, r"dlroW olleH(@)")],
        }],
    };
