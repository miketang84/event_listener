use substrate_subxt::{
    ClientBuilder,
    system::{
        System,
    }
};
use runtime_primitives::generic::Era;
use futures::future::Future;
use futures::stream::Stream;

struct Runtime;

impl System for Runtime {
    type Index = <node_runtime::Runtime as srml_system::Trait>::Index;
    type BlockNumber = <node_runtime::Runtime as srml_system::Trait>::BlockNumber;
    type Hash = <node_runtime::Runtime as srml_system::Trait>::Hash;
    type Hashing = <node_runtime::Runtime as srml_system::Trait>::Hashing;
    type AccountId = <node_runtime::Runtime as srml_system::Trait>::AccountId;
    type Lookup = <node_runtime::Runtime as srml_system::Trait>::Lookup;
    type Header = <node_runtime::Runtime as srml_system::Trait>::Header;
    type Event = <node_runtime::Runtime as srml_system::Trait>::Event;

    type SignedExtra = (
        srml_system::CheckVersion<node_runtime::Runtime>,
        srml_system::CheckGenesis<node_runtime::Runtime>,
        srml_system::CheckEra<node_runtime::Runtime>,
        srml_system::CheckNonce<node_runtime::Runtime>,
        srml_system::CheckWeight<node_runtime::Runtime>,
        srml_balances::TakeFees<node_runtime::Runtime>,
        );

    fn extra(nonce: Self::Index) -> Self::SignedExtra {
        (
            srml_system::CheckVersion::<node_runtime::Runtime>::new(),
            srml_system::CheckGenesis::<node_runtime::Runtime>::new(),
            srml_system::CheckEra::<node_runtime::Runtime>::from(Era::Immortal),
            srml_system::CheckNonce::<node_runtime::Runtime>::from(nonce),
            srml_system::CheckWeight::<node_runtime::Runtime>::new(),
            srml_balances::TakeFees::<node_runtime::Runtime>::from(0),
            )
    }
}

fn main() {
    env_logger::try_init().ok();

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let client_future = ClientBuilder::<Runtime>::new().build();

    let client = rt.block_on(client_future).unwrap();
    let stream = rt.block_on(client.subscribe_events()).unwrap();

    let task = stream.for_each(|t| {
            println!("{:?}", t);
            Ok(())
        })
        .map_err(|_e| ());

    rt.block_on(task).unwrap();
}
