//! Hello-world snippet for `ProgrammingLanguageName::BangAtSharpDollarPercentCaretAndStarPlus`.
//!
//! Imported from leachim6/hello-world (`#/!@#$%^&∗()_+` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BangAtSharpDollarPercentCaretAndStarPlus,
            syntax: r"dlroW olleH(@)",
            tokenized_syntax: &[(TokenKind::Plain, r"dlroW olleH(@)")],
        }],
    };
