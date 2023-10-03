# yew-bootstrap
`yew-bootstrap` is a collection of frontend components made to simplify the usage of [Bootstrap 5](https://getbootstrap.com/docs/5.1/getting-started/introduction/) within the [Yew framework](https://yew.rs/).

<a href="https://crates.io/crates/yew-bootstrap"><img alt="Crate info" src="https://img.shields.io/crates/v/yew-bootstrap.svg" /></a>
<a href="https://docs.rs/yew-bootstrap/"><img alt="API Docs" src="https://img.shields.io/badge/docs.rs-yew--bootstrap-green"/></a>

This project uses [semantic versioning](https://semver.org/).

# Features Implemented
Check the readme for the [`yew-bootstrap` package](/packages/yew-bootstrap/README.md), or the docs.rs link above.

[Our "Basics" Example](https://user-images.githubusercontent.com/163370/225911985-d3928a8a-f8f4-45a0-8906-f554a68c15e5.webm)

# Contributing
Bug reports, feature requests, and pull requests are welcome!

Please try to match your code style to the existing codebase. If you think a change in that style is warranted, feel free to make a suggestion in a new Issue. 

Much of this codebase uses struct components. **For new contributions please use functional components** unless you have good reason to use struct components. It is the [recommended default from Yew](https://yew.rs/docs/concepts/function-components)[^1], and we should be consistent. 
[^1]: > function components - the recommended way to write components when starting with Yew and when writing simple presentation logic.

Please be sure to try `cargo test` before submitting a PR.

All new features should have examples in their documentation via doc strings as well as an example application under `/examples/`.
