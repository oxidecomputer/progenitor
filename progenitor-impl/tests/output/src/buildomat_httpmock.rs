pub mod operations {
    #![doc = r" [`When`](httpmock::When) and [`Then`](httpmock::Then)"]
    #![doc = r" wrappers for each operation. Each can be converted to"]
    #![doc = r" its inner type with a call to `into_inner()`. This can"]
    #![doc = r" be used to explicitly deviate from permitted values."]
    use crate::buildomat_builder::*;
    pub struct ControlHoldWhen(httpmock::When);
    impl ControlHoldWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/control/hold$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct ControlHoldThen(httpmock::Then);
    impl ControlHoldThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: ()) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ControlResumeWhen(httpmock::When);
    impl ControlResumeWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/control/resume$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct ControlResumeThen(httpmock::Then);
    impl ControlResumeThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self) -> Self {
            Self(self.0.status(200u16))
        }
    }

    pub struct TaskGetWhen(httpmock::When);
    impl TaskGetWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/task/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn task(self, value: &str) -> Self {
            let re = regex::Regex::new(&format!("^/v1/task/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct TaskGetThen(httpmock::Then);
    impl TaskGetThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Task) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct TasksGetWhen(httpmock::When);
    impl TasksGetWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/tasks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct TasksGetThen(httpmock::Then);
    impl TasksGetThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &Vec<types::Task>) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct TaskSubmitWhen(httpmock::When);
    impl TaskSubmitWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/tasks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::TaskSubmit) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct TaskSubmitThen(httpmock::Then);
    impl TaskSubmitThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::TaskSubmitResult) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct TaskEventsGetWhen(httpmock::When);
    impl TaskEventsGetWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/tasks/[^/]*/events$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn task(self, value: &str) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/tasks/{}/events$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn minseq<T>(self, value: T) -> Self
        where
            T: Into<Option<u32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("minseq", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "minseq"))
                        .is_none()
                }))
            }
        }
    }

    pub struct TaskEventsGetThen(httpmock::Then);
    impl TaskEventsGetThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &Vec<types::TaskEvent>) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct TaskOutputsGetWhen(httpmock::When);
    impl TaskOutputsGetWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/tasks/[^/]*/outputs$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn task(self, value: &str) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/tasks/{}/outputs$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct TaskOutputsGetThen(httpmock::Then);
    impl TaskOutputsGetThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &Vec<types::TaskOutput>) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct TaskOutputDownloadWhen(httpmock::When);
    impl TaskOutputDownloadWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/tasks/[^/]*/outputs/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn task(self, value: &str) -> Self {
            let re = regex::Regex::new(&format!("^/v1/tasks/{}/outputs/.*$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn output(self, value: &str) -> Self {
            let re = regex::Regex::new(&format!("^/v1/tasks/.*/outputs/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct TaskOutputDownloadThen(httpmock::Then);
    impl TaskOutputDownloadThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn success(self, status: u16, value: serde_json::Value) -> Self {
            assert_eq!(status / 100u16, 2u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct UserCreateWhen(httpmock::When);
    impl UserCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/users$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::UserCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct UserCreateThen(httpmock::Then);
    impl UserCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::UserCreateResult) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct WhoamiWhen(httpmock::When);
    impl WhoamiWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/whoami$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct WhoamiThen(httpmock::Then);
    impl WhoamiThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::WhoamiResult) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct WhoamiPutNameWhen(httpmock::When);
    impl WhoamiPutNameWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/whoami/name$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: String) -> Self {
            Self(self.0.body(value))
        }
    }

    pub struct WhoamiPutNameThen(httpmock::Then);
    impl WhoamiPutNameThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self) -> Self {
            Self(self.0.status(200u16))
        }
    }

    pub struct WorkerBootstrapWhen(httpmock::When);
    impl WorkerBootstrapWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/worker/bootstrap$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::WorkerBootstrap) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct WorkerBootstrapThen(httpmock::Then);
    impl WorkerBootstrapThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::WorkerBootstrapResult) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct WorkerPingWhen(httpmock::When);
    impl WorkerPingWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/worker/ping$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct WorkerPingThen(httpmock::Then);
    impl WorkerPingThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::WorkerPingResult) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct WorkerTaskAppendWhen(httpmock::When);
    impl WorkerTaskAppendWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/worker/task/[^/]*/append$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn task(self, value: &str) -> Self {
            let re = regex::Regex::new(&format!("^/v1/worker/task/{}/append$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::WorkerAppendTask) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct WorkerTaskAppendThen(httpmock::Then);
    impl WorkerTaskAppendThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self) -> Self {
            Self(self.0.status(201u16))
        }
    }

    pub struct WorkerTaskUploadChunkWhen(httpmock::When);
    impl WorkerTaskUploadChunkWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/worker/task/[^/]*/chunk$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn task(self, value: &str) -> Self {
            let re = regex::Regex::new(&format!("^/v1/worker/task/{}/chunk$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: serde_json::Value) -> Self {
            Self(self.0.json_body(value))
        }
    }

    pub struct WorkerTaskUploadChunkThen(httpmock::Then);
    impl WorkerTaskUploadChunkThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::UploadedChunk) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct WorkerTaskCompleteWhen(httpmock::When);
    impl WorkerTaskCompleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/worker/task/[^/]*/complete$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn task(self, value: &str) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/worker/task/{}/complete$", value.to_string()))
                    .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::WorkerCompleteTask) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct WorkerTaskCompleteThen(httpmock::Then);
    impl WorkerTaskCompleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self) -> Self {
            Self(self.0.status(200u16))
        }
    }

    pub struct WorkerTaskAddOutputWhen(httpmock::When);
    impl WorkerTaskAddOutputWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/worker/task/[^/]*/output$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn task(self, value: &str) -> Self {
            let re = regex::Regex::new(&format!("^/v1/worker/task/{}/output$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::WorkerAddOutput) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct WorkerTaskAddOutputThen(httpmock::Then);
    impl WorkerTaskAddOutputThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self) -> Self {
            Self(self.0.status(201u16))
        }
    }

    pub struct WorkersListWhen(httpmock::When);
    impl WorkersListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/workers$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct WorkersListThen(httpmock::Then);
    impl WorkersListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::WorkersResult) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct WorkersRecycleWhen(httpmock::When);
    impl WorkersRecycleWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/workers/recycle$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct WorkersRecycleThen(httpmock::Then);
    impl WorkersRecycleThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self) -> Self {
            Self(self.0.status(200u16))
        }
    }
}

#[doc = r" An extension trait for [`MockServer`](httpmock::MockServer) that"]
#[doc = r" adds a method for each operation. These are the equivalent of"]
#[doc = r" type-checked [`mock()`](httpmock::MockServer::mock) calls."]
pub trait MockServerExt {
    fn control_hold<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ControlHoldWhen, operations::ControlHoldThen);
    fn control_resume<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ControlResumeWhen, operations::ControlResumeThen);
    fn task_get<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::TaskGetWhen, operations::TaskGetThen);
    fn tasks_get<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::TasksGetWhen, operations::TasksGetThen);
    fn task_submit<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::TaskSubmitWhen, operations::TaskSubmitThen);
    fn task_events_get<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::TaskEventsGetWhen, operations::TaskEventsGetThen);
    fn task_outputs_get<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::TaskOutputsGetWhen, operations::TaskOutputsGetThen);
    fn task_output_download<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::TaskOutputDownloadWhen, operations::TaskOutputDownloadThen);
    fn user_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UserCreateWhen, operations::UserCreateThen);
    fn whoami<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WhoamiWhen, operations::WhoamiThen);
    fn whoami_put_name<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WhoamiPutNameWhen, operations::WhoamiPutNameThen);
    fn worker_bootstrap<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkerBootstrapWhen, operations::WorkerBootstrapThen);
    fn worker_ping<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkerPingWhen, operations::WorkerPingThen);
    fn worker_task_append<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkerTaskAppendWhen, operations::WorkerTaskAppendThen);
    fn worker_task_upload_chunk<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkerTaskUploadChunkWhen, operations::WorkerTaskUploadChunkThen);
    fn worker_task_complete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkerTaskCompleteWhen, operations::WorkerTaskCompleteThen);
    fn worker_task_add_output<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkerTaskAddOutputWhen, operations::WorkerTaskAddOutputThen);
    fn workers_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkersListWhen, operations::WorkersListThen);
    fn workers_recycle<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkersRecycleWhen, operations::WorkersRecycleThen);
}

impl MockServerExt for httpmock::MockServer {
    fn control_hold<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ControlHoldWhen, operations::ControlHoldThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ControlHoldWhen::new(when),
                operations::ControlHoldThen::new(then),
            )
        })
    }

    fn control_resume<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ControlResumeWhen, operations::ControlResumeThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ControlResumeWhen::new(when),
                operations::ControlResumeThen::new(then),
            )
        })
    }

    fn task_get<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::TaskGetWhen, operations::TaskGetThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::TaskGetWhen::new(when),
                operations::TaskGetThen::new(then),
            )
        })
    }

    fn tasks_get<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::TasksGetWhen, operations::TasksGetThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::TasksGetWhen::new(when),
                operations::TasksGetThen::new(then),
            )
        })
    }

    fn task_submit<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::TaskSubmitWhen, operations::TaskSubmitThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::TaskSubmitWhen::new(when),
                operations::TaskSubmitThen::new(then),
            )
        })
    }

    fn task_events_get<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::TaskEventsGetWhen, operations::TaskEventsGetThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::TaskEventsGetWhen::new(when),
                operations::TaskEventsGetThen::new(then),
            )
        })
    }

    fn task_outputs_get<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::TaskOutputsGetWhen, operations::TaskOutputsGetThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::TaskOutputsGetWhen::new(when),
                operations::TaskOutputsGetThen::new(then),
            )
        })
    }

    fn task_output_download<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::TaskOutputDownloadWhen, operations::TaskOutputDownloadThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::TaskOutputDownloadWhen::new(when),
                operations::TaskOutputDownloadThen::new(then),
            )
        })
    }

    fn user_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UserCreateWhen, operations::UserCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UserCreateWhen::new(when),
                operations::UserCreateThen::new(then),
            )
        })
    }

    fn whoami<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WhoamiWhen, operations::WhoamiThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WhoamiWhen::new(when),
                operations::WhoamiThen::new(then),
            )
        })
    }

    fn whoami_put_name<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WhoamiPutNameWhen, operations::WhoamiPutNameThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WhoamiPutNameWhen::new(when),
                operations::WhoamiPutNameThen::new(then),
            )
        })
    }

    fn worker_bootstrap<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkerBootstrapWhen, operations::WorkerBootstrapThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WorkerBootstrapWhen::new(when),
                operations::WorkerBootstrapThen::new(then),
            )
        })
    }

    fn worker_ping<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkerPingWhen, operations::WorkerPingThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WorkerPingWhen::new(when),
                operations::WorkerPingThen::new(then),
            )
        })
    }

    fn worker_task_append<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkerTaskAppendWhen, operations::WorkerTaskAppendThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WorkerTaskAppendWhen::new(when),
                operations::WorkerTaskAppendThen::new(then),
            )
        })
    }

    fn worker_task_upload_chunk<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkerTaskUploadChunkWhen, operations::WorkerTaskUploadChunkThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WorkerTaskUploadChunkWhen::new(when),
                operations::WorkerTaskUploadChunkThen::new(then),
            )
        })
    }

    fn worker_task_complete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkerTaskCompleteWhen, operations::WorkerTaskCompleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WorkerTaskCompleteWhen::new(when),
                operations::WorkerTaskCompleteThen::new(then),
            )
        })
    }

    fn worker_task_add_output<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkerTaskAddOutputWhen, operations::WorkerTaskAddOutputThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WorkerTaskAddOutputWhen::new(when),
                operations::WorkerTaskAddOutputThen::new(then),
            )
        })
    }

    fn workers_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkersListWhen, operations::WorkersListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WorkersListWhen::new(when),
                operations::WorkersListThen::new(then),
            )
        })
    }

    fn workers_recycle<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::WorkersRecycleWhen, operations::WorkersRecycleThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WorkersRecycleWhen::new(when),
                operations::WorkersRecycleThen::new(then),
            )
        })
    }
}
