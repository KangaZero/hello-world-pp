//! Hello-world snippet for `ProgrammingLanguageName::Llvm`.
//!
//! Imported from leachim6/hello-world (`l/LLVM.ll` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LLVM.ll>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Llvm => "LLVM",
    versions: [Llvm => "LLVM"],
    extensions: [Ll => "ll"],
}

const LLVM: &str = concat!(
    r#"target datalayout = "e-p:32:32:32-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:32:64-f32:32:32-f64:32:64-v64:64:64-v128:128:128-a0:0:64-f80:128:128"
@.str = internal constant [12 x i8] c""#,
    output_for_hello_world!(),
    r#"\00"

; puts from libc
declare i32 @puts(i8*)

define i32 @main(...) {
	call i32 @puts(i8* getelementptr([12 x i8]* @.str, i32 0, i32 0))
	ret i32 0
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Llvm,
        extensions: &[FileExtension::Ll],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Llvm,
            syntax: LLVM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LLVM,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"target datalayout = "e-p:32:32:32-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:32:64-f32:32:32-f64:32:64-v64:64:64-v128:128:128-a0:0:64-f80:128:128"
@.str = internal constant [12 x i8] c""#,
                            output_for_hello_world!(),
                            r#"\00"

; puts from libc
declare i32 @puts(i8*)

define i32 @main(...) {
	call i32 @puts(i8* getelementptr([12 x i8]* @.str, i32 0, i32 0))
	ret i32 0
}"#
                        ),
                    )],
                )
            },
        }],
    };
