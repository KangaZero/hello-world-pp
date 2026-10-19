//! Hello-world snippet for `ProgrammingLanguageName::PlU2215Sql`.
//!
//! Imported from leachim6/hello-world (`p/PL∕SQL.pls` at `a152253`),
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
    language: PlU2215Sql => "PL∕SQL",
    versions: [PlU2215Sql => "PL∕SQL"],
    extensions: [Pls => "pls"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PlU2215Sql,
        extensions: &[FileExtension::Pls],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PlU2215Sql,
            syntax: concat!(
                r"BEGIN
  dbms_output.put_line('",
                output_for_hello_world!(),
                r"');
END;
/"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"BEGIN
  dbms_output.put_line('",
                    output_for_hello_world!(),
                    r"');
END;
/"
                ),
            )],
        }],
    };
