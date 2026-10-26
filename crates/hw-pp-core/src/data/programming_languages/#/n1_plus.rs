//! Hello-world snippet for `ProgrammingLanguageName::N1Plus`.
//!
//! Imported from leachim6/hello-world (`#/1+` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/1%2B>
//! - <https://esolangs.org/wiki/1+> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: N1Plus => "1+",
    versions: [N1Plus => "1+"],
    extensions: [],
}

const N1_PLUS: &str = r#"11+"""1+"****"; [H]
111++""**1+(D|/"\"/^\)1++; [e]
(D)11+""**++"";; [ll]
111+++"; [o]
"11+"*+; [ ]
"111++"/*\+; [W]
\"; [o]
111+++; [r]
(D)11+""**++; [l]
+; [d]
111++"*1+; [\n]"#;

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N1Plus,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N1Plus,
            syntax: N1_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N1_PLUS,
                    [(
                        TokenKind::Plain,
                        r#"11+"""1+"****"; [H]
111++""**1+(D|/"\"/^\)1++; [e]
(D)11+""**++"";; [ll]
111+++"; [o]
"11+"*+; [ ]
"111++"/*\+; [W]
\"; [o]
111+++; [r]
(D)11+""**++; [l]
+; [d]
111++"*1+; [\n]"#,
                    )],
                )
            },
        }],
    };
