//! Hello-world snippet for `ProgrammingLanguageName::Huf`.
//!
//! Imported from leachim6/hello-world (`h/Huf.huf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Huf => "Huf",
    versions: [Huf => "Huf"],
    extensions: [Huf => "huf"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Huf,
        extensions: &[FileExtension::Huf],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Huf,
            syntax: r"++++++++++|+++++++!++>
++++++++++|++++++++++!+>
++++++++++|++++++++++!++++++++>
++++++++++|++++++++++!++++++++>
++++++++++|+++++++++++!+>
++++++|+++++!++>
++++++++++|++++++++!+++++++>
++++++++++|+++++++++++!+>
++++++++++|+++++++++++!++++>
++++++++++|++++++++++!++++++++>
++++++++++|++++++++++!>",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"++++++++++|+++++++!++>
++++++++++|++++++++++!+>
++++++++++|++++++++++!++++++++>
++++++++++|++++++++++!++++++++>
++++++++++|+++++++++++!+>
++++++|+++++!++>
++++++++++|++++++++!+++++++>
++++++++++|+++++++++++!+>
++++++++++|+++++++++++!++++>
++++++++++|++++++++++!++++++++>
++++++++++|++++++++++!>",
            )],
        }],
    };
