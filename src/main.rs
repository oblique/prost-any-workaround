use foo::Foo;
use prost::Message;

mod foo {
    include!(concat!(env!("OUT_DIR"), "/a.b.c.foo.rs"));
}

trait TypeUrl {
    fn type_url() -> &'static str;
}

fn pack_to_any<M>(msg: M) -> prost_types::Any
where
    M: prost::Message + TypeUrl,
{
    prost_types::Any {
        type_url: M::type_url().to_owned(),
        value: msg.encode_to_vec(),
    }
}

fn unpack_from_any<M>(msg: prost_types::Any) -> Option<M>
where
    M: prost::Message + TypeUrl + Default,
{
    if msg.type_url == M::type_url() {
        Some(M::decode(&msg.value[..]).ok()?)
    } else {
        None
    }
}

fn encoded_foo() -> Vec<u8> {
    Foo {
        msgs: vec![
            pack_to_any(foo::Bar {
                i: 123,
            }),
            pack_to_any(foo::Baz {
                f: 2.3,
            }),
            pack_to_any(foo::Abc {
                b: true,
            }),
            pack_to_any(foo::Bcd {
                some_enum: foo::SomeEnum::C as i32,
            }),
        ],
    }
    .encode_to_vec()
}

fn main() {
    let foo_bytes = encoded_foo();

    let mut foo = Foo::decode(&foo_bytes[..]).unwrap();
    dbg!(unpack_from_any::<foo::Bcd>(foo.msgs.pop().unwrap()).unwrap());
    dbg!(unpack_from_any::<foo::Abc>(foo.msgs.pop().unwrap()).unwrap());
    dbg!(unpack_from_any::<foo::Baz>(foo.msgs.pop().unwrap()).unwrap());
    dbg!(unpack_from_any::<foo::Bar>(foo.msgs.pop().unwrap()).unwrap());
}
