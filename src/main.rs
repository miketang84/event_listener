use substrate_subxt::{
    ClientBuilder,
    DefaultSystem,
};
use runtime_primitives::generic::Era;
use futures::future::Future;
use futures::stream::Stream;

struct Runtime;

impl DefaultSystem<Runtime> for Runtime {}


fn main() {
    env_logger::try_init().ok();

    Runtime::init();

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

