//! Hello-world snippet for `ProgrammingLanguageName::Brainfu4j`.
//!
//! Imported from leachim6/hello-world (PR #1529 `b/helloworld.bfj`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Brainfu4j => "Brainfu4j",
    versions: [Brainfu4j => "Brainfu4j"],
    extensions: [Bfj => "bfj"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainfu4j,
        extensions: &[FileExtension::Bfj],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brainfu4j,
            syntax: r"interface static static static static static static static static public class static static static static static static static static static interface final private class println 
interface static static static static public class static static static static static static static interface final private class static println 
static static static static static static static println println 
static static static println 
interface interface static static static static static static public class static static static static static static static interface final private class static static println 
final final final final final final final final final final final final println 
interface static static static static static static public class static static static static static static static static static interface final private class static println 
class println 
static static static println 
final final final final final final println 
final final final final final final final final println 
interface interface interface static static static static public class static static static static static static static static interface final private class static println ",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"interface static static static static static static static static public class static static static static static static static static static interface final private class println 
interface static static static static public class static static static static static static static interface final private class static println 
static static static static static static static println println 
static static static println 
interface interface static static static static static static public class static static static static static static static interface final private class static static println 
final final final final final final final final final final final final println 
interface static static static static static static public class static static static static static static static static static interface final private class static println 
class println 
static static static println 
final final final final final final println 
final final final final final final final final println 
interface interface interface static static static static public class static static static static static static static static interface final private class static println ",
            )],
        }],
    };
