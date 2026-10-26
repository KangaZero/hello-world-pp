//! Hello-world snippet for `ProgrammingLanguageName::Egl`.
//!
//! Imported from leachim6/hello-world (`e/Egl.egl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Egl.egl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Egl => "Egl",
    versions: [Egl => "Egl"],
    extensions: [Egl => "egl"],
}

const EGL: &str = concat!(
    r#"program HelloWorld
    function main()
        SysLib.writeStdout(""#,
    output_for_hello_world!(),
    r#"");
    end
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Egl,
        extensions: &[FileExtension::Egl],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Egl,
            syntax: EGL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EGL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"program HelloWorld
    function main()
        SysLib.writeStdout(""#,
                            output_for_hello_world!(),
                            r#"");
    end
end"#
                        ),
                    )],
                )
            },
        }],
    };
