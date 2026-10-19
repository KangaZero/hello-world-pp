//! Hello-world snippet for `ProgrammingLanguageName::Boolfuck`.
//!
//! Imported from leachim6/hello-world (`b/Boolfuck` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Boolfuck => "Boolfuck",
    versions: [Boolfuck => "Boolfuck"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Boolfuck,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Boolfuck,
            syntax: r";;;+;+;;+;+;
+;+;+;+;;+;;+;
;;+;;+;+;;+;
;;+;;+;+;;+;
+;;;;+;+;;+;
;;+;;+;+;+;;
;;;;;+;+;;
+;;;+;+;;;+;
+;;;;+;+;;+;
;+;+;;+;;;+;
;;+;;+;+;;+;
;;+;+;;+;;+;
+;+;;;;+;+;;
;+;+;+;",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r";;;+;+;;+;+;
+;+;+;+;;+;;+;
;;+;;+;+;;+;
;;+;;+;+;;+;
+;;;;+;+;;+;
;;+;;+;+;+;;
;;;;;+;+;;
+;;;+;+;;;+;
+;;;;+;+;;+;
;+;+;;+;;;+;
;;+;;+;+;;+;
;;+;+;;+;;+;
+;+;;;;+;+;;
;+;+;+;",
            )],
        }],
    };
