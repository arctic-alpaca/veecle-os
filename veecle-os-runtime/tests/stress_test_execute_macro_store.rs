#![recursion_limit = "256"]
#![allow(unused_variables, non_snake_case, clippy::too_many_arguments)]

//! For stress testing `veecle_os_runtime::execute!` with a large number of data types (but few actors).
//!
//! Performance can be measured with the following commands:
//!
//! ```
//! cargo +nightly rustc --test stress_test_execute_macro_store -- -Ztime-passes
//! nix develop .#nightly --command cargo rustc --test stress_test_execute_macro_store -- -Ztime-passes
//! ```

macro_rules! make_test {
    ($($ident:ident)*) => {
        // Submodule so that we can reuse `$ident` as an identifier-pattern in the actors.
        mod data {
            $(
                #[derive(Copy, Clone, Debug, veecle_os_runtime::Storable)]
                pub(super) struct $ident;
            )*
        }

        #[veecle_os_runtime::actor]
        async fn read_and_discard_all(
            $(
                $ident: veecle_os_runtime::Reader<'_, data::$ident>,
            )*
        ) -> core::convert::Infallible {
            panic!("test completed");
        }

        #[veecle_os_runtime::actor]
        async fn write_all_never(
            $(
                $ident: veecle_os_runtime::Writer<'_, data::$ident>,
            )*
        ) -> core::convert::Infallible {
            panic!("test completed");
        }

        #[test]
        #[should_panic(expected = "test completed")]
        fn stress_test_execute_macro_store() {
            futures::executor::block_on(
                veecle_os_runtime::execute! {
                    store: [$(
                        data::$ident,
                    )*],
                    actors: [
                        ReadAndDiscardAll,
                        WriteAllNever,
                    ],
                }
            );
        }
    }
}

make_test!(A B C D E F G H I J K L M N O P Q R S T U V W X Y Z);
