#![cfg(not(target_arch = "wasm32"))]

use stylus_sdk::{alloy_primitives::*, prelude::MessageAccess};

use libpurrstream::Storage;

#[test]
fn test_does_take_money() {
    let v = stylus_sdk::testing::TestVM::default();
    let mut c = Storage::from(&v);
    c.operator.set(v.msg_sender());
    let d = U256::from(100);
    v.set_value(d);
    let cat = fixed_bytes!("fe886394fe886394");
    assert_eq!(d, c.make_donation(cat, v.msg_sender()).unwrap());
    assert_eq!(d, c.cat_donations.getter(c.epoch_count.get()).get(cat));
}
