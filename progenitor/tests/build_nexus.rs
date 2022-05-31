// Copyright 2021 Oxide Computer Company

progenitor::generate_api!("../sample_openapi/nexus.json");

pub async fn iteration_example() {
    let client = Client::new("xxx");

    let bod = types::LoginParams {
        username: "ahl".to_string(),
    };

    //let mut stream = client.spoof_login(&bod).await.unwrap();

    // loop {
    //     use futures::TryStreamExt;

    //     match stream.try_next().await {
    //         Ok(Some(bytes)) => println!("bytes: {:?}", bytes),
    //         Ok(None) => break,
    //         Err(e) => panic!("{}", e),
    //     }
    // }
}
