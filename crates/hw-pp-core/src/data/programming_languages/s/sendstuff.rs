//! Hello-world snippet for `ProgrammingLanguageName::Sendstuff`.
//!
//! Imported from leachim6/hello-world (`s/SendStuff.ss` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SendStuff.ss>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Sendstuff => "SendStuff",
    versions: [Sendstuff => "SendStuff"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sendstuff,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sendstuff,
            syntax: r">(Add      72 >(Output) 
>(Add      29 >(Output)
>(Add      7  >(Output) 
              >(Output)
>(Add      3  >(Output)
>(Subtract 67 >(Output)
>(Subtract 12 >(Output) 
>(Add      55 >(Output)
>(Add      24 >(Output)
>(Add      3  >(Output)
>(Subtract 6  >(Output)
>(Subtract 8  >(Output)
>(Subtract 67 >(Output)",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r">(Add      72 >(Output) 
>(Add      29 >(Output)
>(Add      7  >(Output) 
              >(Output)
>(Add      3  >(Output)
>(Subtract 67 >(Output)
>(Subtract 12 >(Output) 
>(Add      55 >(Output)
>(Add      24 >(Output)
>(Add      3  >(Output)
>(Subtract 6  >(Output)
>(Subtract 8  >(Output)
>(Subtract 67 >(Output)",
            )],
        }],
    };
