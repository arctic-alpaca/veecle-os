#[derive(Debug, veecle_os_runtime_macros::Storable)]
pub struct Sensor<T>
where
    T: std::fmt::Debug + 'static,
{
    test: T,
}

#[derive(Debug, veecle_os_runtime_macros::Storable)]
#[storable(data_type = "u8")]
pub struct Sensor1<T>(std::marker::PhantomData<T>);

#[derive(Debug, veecle_os_runtime_macros::Storable)]
#[storable(data_type = "u8")]
pub struct Sensor2<const N: usize>([u8; N]);

#[veecle_os_runtime_macros::actor]
async fn macro_test_actor<T, const N: usize>(
    _sensor0_reader: veecle_os_runtime::Reader<'_, Sensor<T>>,
    _sensor0_writer: veecle_os_runtime::Writer<'_, Sensor<T>>,
    _sensor1_reader: veecle_os_runtime::Reader<'_, Sensor1<T>>,
    _sensor1_writer: veecle_os_runtime::Writer<'_, Sensor1<T>>,
    _sensor2_reader: veecle_os_runtime::Reader<'_, Sensor2<N>>,
    _sensor2_writer: veecle_os_runtime::Writer<'_, Sensor2<N>>,
) -> std::convert::Infallible
where
    T: std::fmt::Debug + 'static,
{
    unreachable!("We only care about the code compiling.");
}

fn main() {
    let _ = veecle_os_runtime::execute! {
        store: [Sensor<u8>],
        actors: [MacroTestActor<u8, 2>],
    };
}
