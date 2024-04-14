# prost 0.11 or lower

This repository shows a workaround for packing/unpacking [`prost_types::Any`] type.

# prost 0.12 or above

Workaround is not needed anymore. Do the following:

* Call [`enable_type_names`] in your `build.rs` to enable generation of [`Name`] trait.
* Use [`from_msg`] to convert a message to `prost_types::Any` type.
* Use [`to_msg`] to convert `prost_types::Any` to a message type.

[`Name`]: https://docs.rs/prost/0.12.4/prost/trait.Name.html
[`enable_type_names`]: https://docs.rs/prost-build/0.12.4/prost_build/struct.Config.html#method.enable_type_names
[`from_msg`]: https://docs.rs/prost-types/0.12.4/prost_types/struct.Any.html#method.from_msg
[`prost_types::Any`]: https://docs.rs/prost-types/0.12.4/prost_types/struct.Any.html
[`to_msg`]: https://docs.rs/prost-types/0.12.4/prost_types/struct.Any.html#method.to_msg
