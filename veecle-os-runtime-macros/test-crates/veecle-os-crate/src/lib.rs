/// Test that runtime macros can be used while depending only on `veecle-os`.

#[derive(Debug, veecle_os::runtime::Storable)]
pub struct Foo;

#[veecle_os::runtime::actor]
pub async fn bar() -> core::convert::Infallible {
    unimplemented!("testing compilation")
}
