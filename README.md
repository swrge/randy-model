# randy-model

[![codecov badge][]][codecov link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

`randy-model` is custom version of [`twilight-model`] for randy.
It define the Discord APIs with few convenience methods on top of them.

These are in a single crate for ease of use, a single point of definition,
and a sort of versioning of the Discord API. Similar to how a database
schema progresses in versions, the definition of the API also progresses in
versions.

The types in this crate are reproducible: deserializing a payload into a
type, serializing it, and then deserializing it again will work.

Defined are a number of modules defining types returned by or owned by
resource categories. For example, `gateway` are types used to interact with
and returned by the gateway API. `guild` contains types owned by the Guild
resource category. These types may be directly returned by, built on top of,
or extended by other crates.

Some models have associated builders, which can be found in the
[`randy-tools`] crate.

## License

[ISC][LICENSE.md]

[LICENSE.md]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[`randy-tools`]: https://github.com/swrge/randy-tools
[`twilight-model`]: https://github.com/twilight-rs/twilight
[`twilight`]: https://docs.rs/twilight
[codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
[codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
[github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
[github link]: https://github.com/twilight-rs/twilight
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[rust badge]: https://img.shields.io/badge/rust-1.79+-93450a.svg?style=for-the-badge&logo=rust
