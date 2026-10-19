//! Hello-world snippet for `ProgrammingLanguageName::Jasmin`.
//!
//! Imported from leachim6/hello-world (`j/Jasmin.j` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/Jasmin.j>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Jasmin => "Jasmin",
    versions: [Jasmin => "Jasmin"],
    extensions: [J => "j"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jasmin,
        extensions: &[FileExtension::J],
        categories: &[Assembly],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jasmin,
            syntax: concat!(
                r#".class public Jasmin
.super java/lang/Object

.method public static main([Ljava/lang/String;)V
	.limit stack 2
	getstatic java/lang/System/out Ljava/io/PrintStream;
	ldc ""#,
                output_for_hello_world!(),
                r#""
	invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
	return	
.end method"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#".class public Jasmin
.super java/lang/Object

.method public static main([Ljava/lang/String;)V
	.limit stack 2
	getstatic java/lang/System/out Ljava/io/PrintStream;
	ldc ""#,
                    output_for_hello_world!(),
                    r#""
	invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
	return	
.end method"#
                ),
            )],
        }],
    };
