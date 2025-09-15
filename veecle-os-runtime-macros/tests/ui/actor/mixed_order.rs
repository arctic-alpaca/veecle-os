#[derive(Debug, PartialEq, Clone, Default, veecle_os_runtime::Storable)]
pub struct Sensor(pub u8);

#[derive(Debug, PartialEq, Clone, Default, veecle_os_runtime::Storable)]
pub struct Actuator(pub u8);

#[veecle_os_runtime_macros::actor]
async fn macro_test_actor(
    _actuator_writer: veecle_os_runtime::Writer<'_, Actuator>,
    _sensor_reader: veecle_os_runtime::Reader<'_, Sensor>,
    _sensor_writer: veecle_os_runtime::Writer<'_, Sensor>,
    _actuator_reader: veecle_os_runtime::ExclusiveReader<'_, Actuator>,
) -> std::convert::Infallible {
    unreachable!("We only care about the code compiling.");
}

fn main() {
    let _ = veecle_os_runtime::execute! {
        store: [Actuator, Sensor],
        actors: [MacroTestActor],
    };
}
