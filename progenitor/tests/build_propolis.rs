// Copyright 2022 Oxide Computer Company

// ensure that the websocket channel used for serial console compiles.
mod propolis_client {
    progenitor::generate_api!(
        spec = "../sample_openapi/propolis-server.json",
        interface = Builder,
        tags = Merged,
    );
}

use propolis_client::Client;

pub fn _ignore() {
    let _ = async {
        let _upgraded: reqwest::Upgraded = Client::new("")
            .instance_serial()
            .send()
            .await
            .unwrap()
            .into_inner();
    };
}
