//! Hello-world snippet for `ProgrammingLanguageName::U02f8SharpUff1fBang`.
//!
//! Imported from leachim6/hello-world (`#/()˸;#？!` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: U02f8SharpUff1fBang => "()˸;#？!",
    versions: [U02f8SharpUff1fBang => "()˸;#？!"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U02f8SharpUff1fBang,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U02f8SharpUff1fBang,
            syntax: r":H:e:l:l:o: :W:o:r:l:d!",
            tokenized_syntax: &[(TokenKind::Plain, r":H:e:l:l:o: :W:o:r:l:d!")],
        }],
    };
