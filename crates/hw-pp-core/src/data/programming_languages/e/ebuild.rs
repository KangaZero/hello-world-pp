//! Hello-world snippet for `ProgrammingLanguageName::Ebuild`.
//!
//! Imported from leachim6/hello-world (`e/EBuild.ebuild` at `a152253`),
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
    language: Ebuild => "EBuild",
    versions: [Ebuild => "EBuild"],
    extensions: [Ebuild => "ebuild"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ebuild,
        extensions: &[FileExtension::Ebuild],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ebuild,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
