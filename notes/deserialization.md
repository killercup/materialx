# Notes on Deserializer Implementation

As `.mtlx` files are XML and we are writing this in Rust,
the [quick-xml] crate seemed to be a good choice for parsing.
It offers a [serde] compatible API,
so in theory we can defined some structs and enums,
derive `Deserialize` on them,
and then quick-xml will parse the XML into those structs.
In reality, the XML is a bit more complex than that.
The MaterialX spec is quite large and complex,
and we couldn't find an XSD (schema definition) file for it.
We also don't want to parse the entire spec (at first).

So we decided to start with a small subset of the spec,
and write types that make up what a good AST for it would look like.
For some part, we could actually use the derive macros as it
but for other types we had to write custom deserialization code.
Here are some notes on the ways we did the custom parts.

[quick-xml]: https://docs.rs/quick-xml/0.34.0/
[serde]: https://serde.rs/
[data types]: https://github.com/AcademySoftwareFoundation/MaterialX/blob/v1.39.0/documents/Specification/MaterialX.Specification.md#materialx-data-types

## Well-typed enums with "Unknown" variants

The MaterialX spec has a lot of fields that we'd want to represent as Rust enums instead of strings.
We must assume they are non-exhaustive, though.
For example, we might know the built-in types
[from the spec][data types]
but you can also define custom types.
So we'll define an enum that has a final variant like `Other(String)`.

As of 2024-06-26, this is done manually,
but in the future we might use strum's [`EnumString`] derive and `default` attribute.

[`EnumString`]: https://docs.rs/strum/0.26.3/strum/derive.EnumString.html

## Custom deserializer using intermediate types

One of the tricky elements in the spec is [`<input>`].
It has a `type` attribute that can be one of the [data types] or a custom one.
It can have a `value` attribute which is a constant that can be parsed as the data type.
It can also connect to the output of another node
(using `nodename` or `nodegraph` and selecting the output by name with `output`).
Ideally, we'd be able to make a type that can represent all of these possibilities as a nice enum.

As this seemed quite hard with the derive macros,
we had the idea of writing a custom derive that first tries to deserialize the element into a hash map
and then we could construct our type by looking at the keys in the hash map.
Instead of a hash map we could also define a simpler struct that more closely matches the XML and derive Deserialize on that and then work with that.
(We can even define it inside our Deserialize implementation to hide the type.)

[`<input>`]: https://github.com/AcademySoftwareFoundation/MaterialX/blob/v1.39.0/documents/Specification/MaterialX.Specification.md#inputs

## Good error messages from serde

The default error messages from [serde] and [quick-xml] are not very helpful
as they don't include the path at which the error occurred.
So we just get a generic message like "missing field `input`"
and we have to guess where that field was supposed to be in a file full of `<input>` elements.
Luckily, the [serde_path_to_error] crate exists,
which adds a path like `$value[0].nodegraph` to the error message.
The implementation of `MaterialX::from_str` that we provide already includes this.

[serde_path_to_error]: https://docs.rs/serde_path_to_error/0.1.16/
