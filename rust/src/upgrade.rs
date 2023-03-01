use crate::{Data, UserEventLogs};
use ic_kit::ic;
use ic_kit::macros::{post_upgrade, pre_upgrade};


#[pre_upgrade]
fn pre_upgrade() {
    ic::stable_store((ic::get::<Data>(), ic::get::<UserEventLogs>(), ))
        .expect("Failed to serialize data.");
}

#[post_upgrade]
fn post_upgrade() {
    let (data, logs, ): (Data, UserEventLogs, ) =
        ic::stable_restore().expect("Failed to deserialize.");

    ic::store::<Data>(data);
    ic::store::<UserEventLogs>(logs);
}
