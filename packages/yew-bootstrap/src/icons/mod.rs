//! # Icons
//!
//! All icons are available as a constant, thanks to the build-executed script:
//! ```
//! # use yew::html;
//! # use yew_bootstrap::icons::BI;
//! let icon = BI::HEART;
//! # let result =
//! html!{
//!     <h1>{"I"} {icon} {BI::GEAR}</h1>
//! }
//! # ;
//! ```
//!
//! # Required CSS
//!
//! These icons require the inclusion of a CSS file from Bootstrap (`bootstrap-icons.css`). This file can be added by:
//!
//! 1. Using [`BIFiles::cdn()`](crate::icons::BIFiles::cdn) (easiest, see below)
//! 2. Using [`BIFiles::copy()`](crate::icons::BIFiles::copy) - in build-executed code (recommended, see below)
//! 3. Copying it yourself from the Bootstrap website
//! 4. Accessing the data via [`BIFiles::FILES`](crate::icons::BIFiles::FILES) and delivering it yourself
//!
//! # 1. Quickstart - Using CDN
//!
//! Call `BIFiles::cdn()` inside the `html!{}` returned by your application.
//!
//! # 2. Recommended - Automatically Copying Files
//!
//! This is copies the files to the `dist` directory, which is recommended.
//!
//! It is shown in `/examples/icons`.
//!
//! A copy of `bootstrap-icons` is included and should change only rarely. `trunk` does not add a hash to generated files,
//! and thus a change in those files won't be detected by `trunk`.
//!
//! 1. `Cargo.toml`
//!
//!    Add the build-executed binary.
//!
//!    ```toml
//!    [[bin]]
//!    name = "copy-bootstrap-icons"
//!    ```
//!
//! 2. `src/bin/copy-bootstrap-icons.rs`
//!
//!    Create the program to copy the files.
//!
//!    ```no_run
//!    use std::path::PathBuf;
//!    use yew_bootstrap::icons::BIFiles;
//!
//!    fn main() -> Result<(), std::io::Error> {
//!        let path = PathBuf::from(
//!            std::env::var("TRUNK_STAGING_DIR").expect("Environment variable TRUNK_STAGING_DIR"),
//!        )
//!        .join(BIFiles::NAME);
//!        if !path.is_dir() {
//!            std::fs::create_dir(&path)?;
//!        }
//!        BIFiles::copy(&path)
//!    }
//!    ```
//!
//! 3. `index.html`
//!
//!    Set base reference, link the required CSS and specify your WASM program[^1].
//!
//!    [^1]: Since we'll be writing a build-executed program, there are now two binaries and trunk needs to know which is your WASM binary.
//!
//!    ```html
//!    <base data-trunk-public-url />
//!    <link rel="stylesheet" href="bootstrap-icons-v1.11.3/bootstrap-icons.css" />
//!    <link data-trunk rel="rust" data-bin="name-of-app" />
//!    ```
//!
//! 4. `Trunk.toml`
//!
//!    Add a hook to run the build-executed program.
//!
//!    ```toml
//!    [[hooks]]
//!    stage = "build"
//!    command = "cargo"
//!    command_arguments = ["run", "--bin", "copy-bootstrap-icons"]
//!    ```
//!
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::pedantic)]

use core::hash::{Hash, Hasher};
use yew::html::{ChildrenRenderer, IntoPropValue};
use yew::virtual_dom::{VNode, VRaw};
use yew::{AttrValue, Html};

/// Represents an bootstrap-icon.
///
/// This is a transparent wrapper of a `&'static str`, so `Copy` is cheap.
///
/// Use [`BI::html`] or the `From` or `IntoPropValue` implementation to generate the actual html.
///
/// Search for an icon at <https://icons.getbootstrap.com/#search>.
// Invariant: All possible strings are different and thus `(ptr,len)` must me different as well.
// Invariant: No two strings at different pointers are equal,
// Invariant: this is guaranteed due to the fact that it's not possible to create new.
#[derive(Clone, Copy, Ord, PartialOrd, Eq)]
#[repr(transparent)]
pub struct BI(pub(crate) &'static str);

impl BI {
    /// Returns the `Html` of this icon.
    #[inline]
    pub const fn html(self) -> Html {
        VNode::VRaw(VRaw {
            html: AttrValue::Static(self.0),
        })
    }

    /// Returns the raw html as a str of this icon.
    #[inline]
    #[must_use]
    pub const fn raw_html(self) -> &'static str {
        self.0
    }
}

impl PartialEq for BI {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // Invariant: All possible strings are different and thus `(ptr,len)` must me different as well.
        // Invariant: No two strings at different pointers are equal,
        // Invariant: this is guaranteed due to the fact that it's not possible to create new.
        // Performance hack: Only check those.
        self.0.as_ptr() as usize == other.0.as_ptr() as usize && self.0.len() == other.0.len()
    }
}

impl Hash for BI {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Invariant: All possible strings are different and thus `(ptr,len)` must me different as well.
        // Invariant: No two strings at different pointers are equal,
        // Invariant: this is guaranteed due to the fact that it's not possible to create new.
        // Performance hack: Only hash the ptr to the middle of the string.
        (self.0.as_ptr() as usize + (self.0.len() >> 1)).hash(state);
    }
}

impl From<BI> for Html {
    #[inline]
    fn from(value: BI) -> Self {
        value.html()
    }
}

impl From<&BI> for Html {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn from(value: &BI) -> Self {
        Html::from(*value)
    }
}

impl IntoPropValue<ChildrenRenderer<VNode>> for BI {
    #[inline]
    fn into_prop_value(self) -> ChildrenRenderer<VNode> {
        self.html().into_prop_value()
    }
}

impl IntoPropValue<ChildrenRenderer<VNode>> for &BI {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn into_prop_value(self) -> ChildrenRenderer<VNode> {
        (*self).into_prop_value()
    }
}

impl IntoPropValue<Html> for BI {
    #[inline]
    fn into_prop_value(self) -> Html {
        self.html()
    }
}

impl IntoPropValue<Html> for &BI {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn into_prop_value(self) -> Html {
        (*self).into_prop_value()
    }
}



// // ToHtml removed in yew 0.22
// impl ToHtml for BI {
//     #[allow(clippy::inline_always)]
//     #[inline(always)]
//     fn to_html(&self) -> Html {
//         self.html()
//     }
// }

/// Holds all bootstrap-icons data.
///
/// Intended use:
/// ```
/// # use yew_bootstrap::icons::BIFiles;
/// let BIFiles {css, font_woff, font_woff2, license} = BIFiles::FILES;
/// ```
/// (That way it will be an error if a file is added/removed.)
pub struct BIFiles {
    /// Contents of the file `bootstrap-icons.css`.
    pub css: &'static str,
    /// Contents of the file `fonts/bootstrap-icons.woff`.
    pub font_woff: &'static [u8],
    /// Contents of the file `fonts/bootstrap-icons.woff2`.
    pub font_woff2: &'static [u8],
    /// Contents of the file `fonts/LICENSE`.
    pub license: &'static str,
}

/// allows compile time concatenation with other strings to make const 'static str
macro_rules! version {
    () => {
        "v1.11.3"
    };
}
/// provides a resuable path to the bootstrap-icons files that we can make const 'static str with
macro_rules! path {
    () => {
        concat!("../../bootstrap-icons-", version!(), "/")
    };
}

impl BIFiles {
    /// Version of the package.
    pub const VERSION: &'static str = version!();

    /// Name of the package.
    pub const NAME: &'static str = concat!("bootstrap-icons-", version!());

    /// All bootstrap-icons files.
    ///
    /// Intended use:
    /// ```
    /// # use yew_bootstrap::icons::BIFiles;
    /// let BIFiles {css, font_woff, font_woff2, license} = BIFiles::FILES;
    /// ```
    /// (That way it will be an error if a file is added/removed.)
    pub const FILES: Self = Self {
        css: include_str!(concat!(path!(), "bootstrap-icons.css")),
        font_woff: include_bytes!(concat!(path!(), "fonts/bootstrap-icons.woff")),
        font_woff2: include_bytes!(concat!(path!(), "fonts/bootstrap-icons.woff2")),
        license: include_str!(concat!(path!(), "fonts/LICENSE")),
    };

    /// Load the bootstrap-icons files from the official cdn.
    ///
    /// Call `BIFiles::cdn()` inside the `html!{}` returned by your application.
    pub const fn cdn() -> VNode {
        VNode::VRaw(VRaw {
            html: AttrValue::Static(concat!(
                r#"<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap-icons@"#,
                version!(),
                r#"/font/bootstrap-icons.css">"#
            )),
        })
    }

    /// Copy all bootstrap icons files to the specified directory.
    ///
    /// # Errors
    ///
    /// Will return an error when there is a problem with creating the directories or writing the files.
    pub fn copy(to: &std::path::Path) -> Result<(), std::io::Error> {
        let BIFiles {
            css,
            font_woff,
            font_woff2,
            license,
        } = Self::FILES;

        let fonts = to.join("fonts");
        if !fonts.is_dir() {
            std::fs::create_dir(&fonts)?;
        }
        std::fs::write(to.join("bootstrap-icons.css"), css)?;
        std::fs::write(fonts.join("bootstrap-icons.woff"), font_woff)?;
        std::fs::write(fonts.join("bootstrap-icons.woff2"), font_woff2)?;
        std::fs::write(fonts.join("LICENSE"), license)?;

        Ok(())
    }
}

include!(concat!(env!("OUT_DIR"), "/bootstrap_icons_generated.rs"));
