# Pretty Goodness

String to String prettification for text based data serialisation formats (only JSON is supported currently).

For data serialisation [Serde](https://serde.rs/) is the gold standard.

However if you get some serialised data and it needs to be prettyafied Serde would probably be the default choice for doing this.

To do the prettification Serde Serialises and then deserialises the data from and to text based formats.

Wouldn't it be more efficient if the serialisation and deserialisation steps could be avoided?

This is what Pretty Goodness is for: the conversion of ugly Strings into a pretty Strings with no intermediate steps.

## Todo:

- Add code examples.
- Add functionality to prettifiy other text formats such as TOML, YAML and RON.
- Move the Indentor, ScopedIndentor and AutoOutdenter to Corlib as well as any other related objects.
- Complete documentation
- Clean up the code base.

## Coding Style

This project uses a coding style the emphasises the use of white space over keeping the line and column counts as low as possible.

So this:

```rust
fn foo()
{

    bar();

}

```

Not this:

```rust
fn foo()
{
    bar();
}

```

<br/>

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0 (see also: https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT (see also: https://www.tldrlegal.com/license/mit-license))

at your discretion

<br/>

## Contributing

Please clone the repository and create an issue explaining what feature or features you'd like to add or bug or bugs you'd like to fix and perhaps how you intend to implement these additions or fixes. Try to include details though it doesn't need to be exhaustive and we'll take it from there (dependant on availability).

<br/>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.



