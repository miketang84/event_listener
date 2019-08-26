use substrate_subxt::{
    ClientBuilder,
    system::BasicSystem,
    balances::{
        self,
        Balances,
        BalancesXt,
        BasicBalances,
    }
};
use runtime_primitives::generic::Era;
use futures::future::Future;
use futures::stream::Stream;
use substrate_keyring::AccountKeyring;
use substrate_primitives::crypto::Pair;

struct Runtime;

impl BasicSystem for Runtime {}
impl BasicBalances for Runtime {}


fn main() {
    env_logger::try_init().ok();

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let client_future = ClientBuilder::<Runtime>::new().build();

    let client = rt.block_on(client_future).unwrap();
    let signer = AccountKeyring::Alice.pair();
    let mut xt = rt.block_on(client.xt(signer, None)).unwrap();

    let dest = AccountKeyring::Bob.pair().public();
    let transfer = xt
        .balances(|calls| balances::transfer::<Runtime>(calls, dest.clone().into(), 10))
        .submit();
    rt.block_on(transfer).unwrap();

}

