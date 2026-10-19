//! Hello-world snippet for `ProgrammingLanguageName::Cil`.
//!
//! Imported from leachim6/hello-world (`c/Cil.il` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cil.il>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cil => "Cil",
    versions: [Cil => "Cil"],
    extensions: [Il => "il"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cil,
        extensions: &[FileExtension::Il],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cil,
            syntax: concat!(
                r#"// ilasm cil.il
.assembly HelloWorld {}
.method public static void Main() cil managed
{
     .entrypoint
     .maxstack 1
     ldstr ""#,
                output_for_hello_world!(),
                r#""
     call void [mscorlib]System.Console::WriteLine(string)
     ret
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"// ilasm cil.il
.assembly HelloWorld {}
.method public static void Main() cil managed
{
     .entrypoint
     .maxstack 1
     ldstr ""#,
                    output_for_hello_world!(),
                    r#""
     call void [mscorlib]System.Console::WriteLine(string)
     ret
}"#
                ),
            )],
        }],
    };
