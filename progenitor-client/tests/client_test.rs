// Copyright 2023 Oxide Computer Company

use progenitor_client::encode_path;

#[test]
fn test_path_segment_encoding() {
    assert_eq!(encode_path("192.168.0.0/24"), "192.168.0.0%2F24");
}
