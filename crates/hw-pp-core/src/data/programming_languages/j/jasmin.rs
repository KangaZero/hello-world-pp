//! Hello-world snippet for `ProgrammingLanguageName::Jasmin`.
//!
//! Imported from leachim6/hello-world (`j/Jasmin.j` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/Jasmin.j>
//! - <https://www.wikidata.org/wiki/Q740158> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Jasmin => "Jasmin",
    versions: [Jasmin => "Jasmin"],
    extensions: [J => "j"],
}

const JASMIN: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jasmin,
        extensions: &[FileExtension::J],
        categories: &[Assembly],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jasmin,
            syntax: JASMIN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JASMIN,
                    [
                        (TokenKind::Macro, r".class"),
                        (
                            TokenKind::Plain,
                            r" public Jasmin
",
                        ),
                        (TokenKind::Macro, r".super"),
                        (
                            TokenKind::Plain,
                            r" java/lang/Object

",
                        ),
                        (TokenKind::Macro, r".method"),
                        (TokenKind::Plain, r" public static main"),
                        (TokenKind::Punctuation, r"(["),
                        (TokenKind::Plain, r"Ljava/lang/"),
                        (TokenKind::Keyword, r"String"),
                        (TokenKind::Comment, r";)V"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Macro, r".limit"),
                        (TokenKind::Plain, r" stack "),
                        (TokenKind::Number, r"2"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"getstatic"),
                        (
                            TokenKind::Plain,
                            r" java/lang/System/out Ljava/io/PrintStream",
                        ),
                        (TokenKind::Comment, r";"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"ldc"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"invokevirtual"),
                        (TokenKind::Plain, r" java/io/PrintStream/"),
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"Ljava/lang/"),
                        (TokenKind::Keyword, r"String"),
                        (TokenKind::Comment, r";)V"),
                        (
                            TokenKind::Plain,
                            r"
	",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (
                            TokenKind::Plain,
                            r"	
",
                        ),
                        (TokenKind::Macro, r".end"),
                        (TokenKind::Plain, r" method"),
                    ],
                )
            },
        }],
    };
