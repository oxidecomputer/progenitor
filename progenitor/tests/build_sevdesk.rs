// Copyright 2022 Oxide Computer Company

mod positional {
    use self::types::VoucherUploadFileForm;

    progenitor::generate_api!("../sample_openapi/sevdesk-san-sub.yaml");

    fn _ignore() {
        let _ = Client::new("").voucher_upload_file(&VoucherUploadFileForm { file: Some("foo".to_owned()) });
    }
}

mod builder_untagged {
    use self::types::VoucherUploadFileForm;

    progenitor::generate_api!(
        spec = "../sample_openapi/sevdesk-san-sub.yaml",
        interface = Builder,
        tags = Merged,
    );

    fn _ignore() {
        let _ = Client::new("")
            .voucher_upload_file()
            .body(&VoucherUploadFileForm { file: Some("foo".to_owned()) })
            .send();
    }
}

mod builder_tagged {
    use self::types::VoucherUploadFileForm;

    progenitor::generate_api!(
        spec = "../sample_openapi/sevdesk-san-sub.yaml",
        interface = Builder,
        tags = Separate,
    );

    fn _ignore() {
        let _ = Client::new("")
            .voucher_upload_file()
            .body(&VoucherUploadFileForm { file: Some("foo".to_owned()) })
            .send();
    }
}
