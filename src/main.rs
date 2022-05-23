use ic_cdk::export::candid::candid_method;
use ic_cdk::export::Principal;
use ic_cdk_macros::{query, update};
use std::cell::RefCell;

thread_local!(
    static COUNTER: RefCell<u64> = RefCell::new(0);
);

#[query]
#[candid_method(query)]
fn query() -> u64 {
    COUNTER.with(|c| c.borrow().clone())
}

#[update]
#[candid_method(update)]
async fn update() {
    COUNTER.with(|c| {
        *c.borrow_mut() += 1;
    });

    ic_cdk::block_on(async {
        // usually, when the canister is trapped, it will auto rollback the state
        // ic_cdk::trap("panic!");

        // some calls
        let (rand,): (Vec<u8>,) = ic_cdk::call(Principal::management_canister(), "raw_rand", ())
            .await
            .unwrap();

        ic_cdk::println!("{:?}", rand);

        // if the panic occurred after the async/await calls, the atomicity doesn't guarantee here
        // we have to rollback manually
        // ic_cdk::trap("panic!");

        // rollback
        COUNTER.with(|c| {
            *c.borrow_mut() -= 1;
        });
    });
}

#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    ic_cdk::export::candid::export_service!();
    std::print!("{}", __export_service());
}
