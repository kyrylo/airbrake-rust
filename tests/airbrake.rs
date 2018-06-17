extern crate airbrake;
extern crate backtrace;
extern crate mockito;

use backtrace::Backtrace;
use mockito::{mock, Matcher};

#[test]
fn it_notifies_airbrake() {
    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: "81bbff95d52f8856c770bb39e827f3f6",
        host: mockito::SERVER_URL,
        ..Default::default()
    });

    let error = "xc".parse::<u32>().err().unwrap();
    let notice = notifier.build_notice(error);
    let mock = mock("POST", "/api/v3/projects/113743/notices")
        .match_header("content-type", "application/json")
        .create();
    notifier.notify(notice).expect("notifier.notify failed");
    mock.assert();
}

#[test]
fn it_attaches_a_backtrace() {
    let backtrace = Backtrace::new();

    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: "81bbff95d52f8856c770bb39e827f3f6",
        host: mockito::SERVER_URL,
        ..Default::default()
    });

    let error = "xc".parse::<u32>().err().unwrap();
    let notice = notifier.build_notice(error).set_backtrace(backtrace);

    let mock = mock("POST", "/api/v3/projects/113743/notices")
        .match_body(Matcher::Regex(String::from(
            r#""backtrace":\[\{"line":\d+,"file":".+","function":".+".*\}\]"#,
        )))
        .create();
    notifier.notify(notice).expect("notifier.notify failed");
    mock.assert();
}

#[test]
fn it_attaches_params() {
    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: "81bbff95d52f8856c770bb39e827f3f6",
        host: mockito::SERVER_URL,
        ..Default::default()
    });

    let error = "xc".parse::<u32>().err().unwrap();
    let mut params = std::collections::HashMap::new();
    params.insert(String::from("mango"), airbrake::Param::Int32(42));
    params.insert(
        String::from("banana"),
        airbrake::Param::String(String::from("tasty")),
    );

    let notice = notifier.build_notice(error).set_params(params);

    let mock = mock("POST", "/api/v3/projects/113743/notices")
        .match_body(Matcher::Regex(String::from(
            r#""params":\{"banana":\{"String":"tasty"\},"mango":\{"Int32":42\}\}"#,
        )))
        .create();
    notifier.notify(notice).expect("notifier.notify failed");
    mock.assert();
}

// #[test]
// fn it_routes_notices_through_a_proxy() {
//     let notifier = airbrake::Notifier::new(airbrake::Config {
//         project_id: 113743,
//         project_key: "81bbff95d52f8856c770bb39e827f3f6",
//         proxy_url: "http://localhost:8080",
//         host: mockito::SERVER_URL,
//         ..Default::default()
//     });

//     let error = "xc".parse::<u32>().err().unwrap();
//     let notice = notifier.build_notice(error);
//     let mock = mock("POST", "/api/v3/projects/113743/notices")
//         .with_header("proxy-authenticate", "basic")
//         .create();
//     let resp = notifier.notify(notice).expect("notifier.notify failed");
//     println!("{:#?}", resp);
//     mock.assert();
// }
