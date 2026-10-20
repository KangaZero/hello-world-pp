//! Hello-world snippet for `ProgrammingLanguageName::Parenthetic`.
//!
//! Imported from leachim6/hello-world (`p/Parenthetic.p` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Parenthetic.p>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Parenthetic => "Parenthetic",
    versions: [Parenthetic => "Parenthetic"],
    extensions: [P => "p"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Parenthetic,
        extensions: &[FileExtension::P],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Parenthetic,
            syntax: r"Parenthetic - https://github.com/cammckinnon/Parenthetic

((()()())(()(()()))((()(()))((())()()()()()()())((()()(()))((())()()()()()()()()())
((())()()()()()()()()()()))))((()()())(()(()()()))((()(())(())())((())()()()()()()(
)()()()()()()()()()()()()()()()()()()()()()()()()())))((()(()))((()(())(())())((()(
()))(()(()()))((())()()()()()()())))((()(())(())())((()(()))(()(()()))((())()()()()
)))((()(())(())())((()(()))(()(()()))((())()()()()()()()()()()())))((()(())(())())(
(()(()))(()(()()))((())()()()()()()()()()()())))((()(())(())())((()(()))(()(()()))(
(())()()()()()()()()()()()()()())))(()(()()()))((()(())(())())((()(()))(()(()()))((
())()()()()()()()()()()()()()()()()()()()()()())))((()(())(())())((()(()))(()(()())
)((())()()()()()()()()()()()()()())))((()(())(())())((()(()))(()(()()))((())()()()(
)()()()()()()()()()()()()())))((()(())(())())((()(()))(()(()()))((())()()()()()()()
()()()())))((()(())(())())((()(()))(()(()()))((())()()()))))",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"Parenthetic - https://github.com/cammckinnon/Parenthetic

((()()())(()(()()))((()(()))((())()()()()()()())((()()(()))((())()()()()()()()()())
((())()()()()()()()()()()))))((()()())(()(()()()))((()(())(())())((())()()()()()()(
)()()()()()()()()()()()()()()()()()()()()()()()()())))((()(()))((()(())(())())((()(
()))(()(()()))((())()()()()()()())))((()(())(())())((()(()))(()(()()))((())()()()()
)))((()(())(())())((()(()))(()(()()))((())()()()()()()()()()()())))((()(())(())())(
(()(()))(()(()()))((())()()()()()()()()()()())))((()(())(())())((()(()))(()(()()))(
(())()()()()()()()()()()()()()())))(()(()()()))((()(())(())())((()(()))(()(()()))((
())()()()()()()()()()()()()()()()()()()()()()())))((()(())(())())((()(()))(()(()())
)((())()()()()()()()()()()()()()())))((()(())(())())((()(()))(()(()()))((())()()()(
)()()()()()()()()()()()()())))((()(())(())())((()(()))(()(()()))((())()()()()()()()
()()()())))((()(())(())())((()(()))(()(()()))((())()()()))))",
            )],
        }],
    };
