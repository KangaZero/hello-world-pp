//! Hello-world snippet for `ProgrammingLanguageName::Ebuild`.
//!
//! Imported from leachim6/hello-world (`e/EBuild.ebuild` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/EBuild.ebuild>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Configuration,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Ebuild => "EBuild",
    versions: [Ebuild => "EBuild"],
    extensions: [Ebuild => "ebuild"],
}

const EBUILD: &str = concat!(
    r#"# Copyright 1999-2008 Gentoo Foundation
# Distributed under the terms of the GNU General Public License v2
# $Header: $

DESCRIPTION=""#,
    output_for_hello_world!(),
    r#""
HOMEPAGE="http://hello-world.org/"
SRC_URI="http://hello-world.org/"

LICENSE="GPL"
SLOT="0"
KEYWORDS="~x86"
IUSE=""

DEPEND=""
RDEPEND=""

src_compile() {
	einfo ""#,
    output_for_hello_world!(),
    r#""
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ebuild,
        extensions: &[FileExtension::Ebuild],
        categories: &[Configuration],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ebuild,
            syntax: EBUILD,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EBUILD,
                    [
                        (
                            TokenKind::Comment,
                            r"# Copyright 1999-2008 Gentoo Foundation",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (
                            TokenKind::Comment,
                            r"# Distributed under the terms of the GNU General Public License v2",
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Comment, r"# $Header: $"),
                        (
                            TokenKind::Plain,
                            r"

DESCRIPTION=",
                        ),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
HOMEPAGE=",
                        ),
                        (TokenKind::String, r#""http://hello-world.org/""#),
                        (
                            TokenKind::Plain,
                            r"
SRC_URI=",
                        ),
                        (TokenKind::String, r#""http://hello-world.org/""#),
                        (
                            TokenKind::Plain,
                            r"

LICENSE=",
                        ),
                        (TokenKind::String, r#""GPL""#),
                        (
                            TokenKind::Plain,
                            r"
SLOT=",
                        ),
                        (TokenKind::String, r#""0""#),
                        (
                            TokenKind::Plain,
                            r"
KEYWORDS=",
                        ),
                        (TokenKind::String, r#""~x86""#),
                        (
                            TokenKind::Plain,
                            r"
IUSE=",
                        ),
                        (TokenKind::String, r#""""#),
                        (
                            TokenKind::Plain,
                            r"

DEPEND=",
                        ),
                        (TokenKind::String, r#""""#),
                        (
                            TokenKind::Plain,
                            r"
RDEPEND=",
                        ),
                        (TokenKind::String, r#""""#),
                        (
                            TokenKind::Plain,
                            r"

src_compile",
                        ),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
	einfo ",
                        ),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
