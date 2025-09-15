#[derive(Debug, PartialEq, Clone, Default, veecle_os_runtime::Storable)]
pub struct Sensor(pub u8);

#[veecle_os_runtime_macros::actor]
async fn macro_test_actor(_reader: veecle_os_runtime::Reader) -> std::convert::Infallible {
    unreachable!("We only care about the code compiling.");
}

#[veecle_os_runtime_macros::actor]
async fn macro_test_actor2(_reader: veecle_os_runtime::Reader) -> std::convert::Infallible {
    unreachable!("We only care about the code compiling.");
}

#[veecle_os_runtime_macros::actor]
async fn macro_test_actor3(_reader: veecle_os_runtime::Reader<Sensor>) -> std::convert::Infallible {
    unreachable!("We only care about the code compiling.");
}

#[veecle_os_runtime_macros::actor]
async fn macro_test_actor4(_reader: veecle_os_runtime::Reader<'b, Sensor>) -> std::convert::Infallible {
    unreachable!("We only care about the code compiling.");
}

#[veecle_os_runtime_macros::actor]
async fn macro_test_actor5(_reader: veecle_os_runtime::Reader) -> std::convert::Infallible {
    unreachable!("We only care about the code compiling.");
}

fn main() {}
