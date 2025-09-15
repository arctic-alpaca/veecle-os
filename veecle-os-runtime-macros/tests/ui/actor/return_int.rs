#[derive(Debug, PartialEq, Clone, Default, veecle_os_runtime::Storable)]
pub struct Sensor(pub u8);

#[veecle_os_runtime_macros::actor]
async fn macro_test_actor(
    _sensor_reader: veecle_os_runtime::Reader<'_, Sensor>,
    _sensor_writer: veecle_os_runtime::Writer<'_, Sensor>,
) -> i32 {
    0
}

fn main() {
    let _ = veecle_os_runtime::execute! {
        store: [Sensor],
        actors: [MacroTestActor],
    };
}
