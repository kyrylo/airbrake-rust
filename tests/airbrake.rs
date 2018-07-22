extern crate airbrake;
extern crate backtrace;
extern crate mockito;

use backtrace::Backtrace;
use mockito::{mock, Matcher};

use std::collections::HashMap;

#[test]
fn test_airbrake_notify() {
    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: "81bbff95d52f8856c770bb39e827f3f6",
        host: mockito::SERVER_URL,
        ..Default::default()
    });

    let error = "xc".parse::<u32>().err().unwrap();
    let mut notice = notifier.build_notice(error);
    let mock = mock("POST", "/api/v3/projects/113743/notices")
        .match_header("content-type", "application/json")
        .match_header("authorization", "Bearer 81bbff95d52f8856c770bb39e827f3f6")
        .create();
    notifier
        .notify(&mut notice)
        .expect("notifier.notify failed");
    mock.assert();
}

#[test]
fn test_backtrace_attachment() {
    let backtrace = Backtrace::new();

    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: "81bbff95d52f8856c770bb39e827f3f6",
        host: mockito::SERVER_URL,
        ..Default::default()
    });

    let error = "xc".parse::<u32>().err().unwrap();
    let mut notice = notifier.build_notice(error);
    notice.set_backtrace(backtrace);

    let mock = mock("POST", "/api/v3/projects/113743/notices")
        .match_body(Matcher::Regex(String::from(
            r#""backtrace":\[\{"line":\d+,"file":".+","function":".+".*\}\]"#,
        )))
        .create();
    notifier
        .notify(&mut notice)
        .expect("notifier.notify failed");
    mock.assert();
}

#[test]
fn test_params_attachment() {
    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: "81bbff95d52f8856c770bb39e827f3f6",
        host: mockito::SERVER_URL,
        ..Default::default()
    });

    let error = "xc".parse::<u32>().err().unwrap();
    let mut params = HashMap::new();
    params.insert(String::from("mango"), airbrake::Param::Int32(42));
    let mut notice = notifier.build_notice(error);
    notice.set_params(params);

    let mock = mock("POST", "/api/v3/projects/113743/notices")
        .match_body(Matcher::Regex(String::from(
            r#""params":\{"mango":\{"Int32":42\}\}"#,
        )))
        .create();
    notifier
        .notify(&mut notice)
        .expect("notifier.notify failed");
    mock.assert();
}

#[test]
fn test_routing_notices_through_a_proxy() {
    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: "81bbff95d52f8856c770bb39e827f3f6",
        proxy_url: mockito::SERVER_URL,
        host: "http://127.0.0.1:8080",
        ..Default::default()
    });

    let error = "xc".parse::<u32>().err().unwrap();
    let mut notice = notifier.build_notice(error);
    let server_mock = mock(
        "POST",
        "http://127.0.0.1:8080/api/v3/projects/113743/notices",
    ).create();
    notifier
        .notify(&mut notice)
        .expect("notifier.notify failed");

    server_mock.assert();
}

#[test]
fn test_app_version_setting() {
    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: "81bbff95d52f8856c770bb39e827f3f6",
        host: mockito::SERVER_URL,
        app_version: "v1.2.3",
        ..Default::default()
    });

    let error = "xc".parse::<u32>().err().unwrap();
    let mut notice = notifier.build_notice(error);

    let mock = mock("POST", "/api/v3/projects/113743/notices")
        .match_body(Matcher::Regex(String::from(r#""version":"v1.2.3""#)))
        .create();
    notifier
        .notify(&mut notice)
        .expect("notifier.notify failed");
    mock.assert();
}

#[test]
fn test_add_filter() {
    let mut notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: "81bbff95d52f8856c770bb39e827f3f6",
        host: mockito::SERVER_URL,
        app_version: "v1.2.3",
        ..Default::default()
    });

    notifier.add_filter(|n| {
        n.set_app_version("v3.2.1");
    });

    let error = "xc".parse::<u32>().err().unwrap();
    let mut notice = notifier.build_notice(error);

    let mock = mock("POST", "/api/v3/projects/113743/notices").create();
    notifier
        .notify(&mut notice)
        .expect("notifier.notify failed");
    assert_eq!(notice.context.version, "v3.2.1");
    mock.assert();
}
