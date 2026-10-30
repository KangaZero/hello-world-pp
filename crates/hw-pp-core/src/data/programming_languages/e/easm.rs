//! Hello-world snippet for `ProgrammingLanguageName::Easm`.
//!
//! Imported from leachim6/hello-world (PR #1824 `e/easm.easm`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1824>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Easm => "easm",
    versions: [Easm => "easm"],
    extensions: [Easm => "easm"],
}

const EASM: &str = concat!(
    r#"subsystem cui

section imports

    from msvcr70.dll import printf
    from kernel32.dll import ExitProcess

section data

    string pszOutput = ""#,
    output_for_hello_world!(),
    r#""

section code

    call printf (&pszOutput)
    call ExitProcess (00h)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Easm,
        extensions: &[FileExtension::Easm],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Easm,
            syntax: EASM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EASM,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"subsystem cui

section imports

    from msvcr70.dll import printf
    from kernel32.dll import ExitProcess

section data

    string pszOutput = ""#,
                            output_for_hello_world!(),
                            r#""

section code

    call printf (&pszOutput)
    call ExitProcess (00h)"#
                        ),
                    )],
                )
            },
        }],
    };
