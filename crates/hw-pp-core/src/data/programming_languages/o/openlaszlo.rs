//! Hello-world snippet for `ProgrammingLanguageName::Openlaszlo`.
//!
//! Imported from leachim6/hello-world (`o/OpenLaszlo.lzx` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Openlaszlo => "OpenLaszlo",
    versions: [Openlaszlo => "OpenLaszlo"],
    extensions: [Lzx => "lzx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Openlaszlo,
        extensions: &[FileExtension::Lzx],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Openlaszlo,
            syntax: concat!(
                r"<canvas>
    <text>",
                output_for_hello_world!(),
                r"</text>
</canvas>"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"<canvas>
    <text>",
                    output_for_hello_world!(),
                    r"</text>
</canvas>"
                ),
            )],
        }],
    };
