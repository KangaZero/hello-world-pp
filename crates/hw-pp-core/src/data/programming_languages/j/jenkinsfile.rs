//! Hello-world snippet for `ProgrammingLanguageName::Jenkinsfile`.
//!
//! Imported from leachim6/hello-world (`j/Jenkinsfile` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Jenkinsfile => "Jenkinsfile",
    versions: [Jenkinsfile => "Jenkinsfile"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jenkinsfile,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jenkinsfile,
            syntax: concat!(
                r"pipeline {
    agent any
    stages {
        stage('Print Message') {
            steps {
                echo '",
                output_for_hello_world!(),
                r"'
            }
        }
    }
}"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"pipeline {
    agent any
    stages {
        stage('Print Message') {
            steps {
                echo '",
                    output_for_hello_world!(),
                    r"'
            }
        }
    }
}"
                ),
            )],
        }],
    };
