extern crate airbrake;

extern crate backtrace;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use backtrace::Backtrace;

#[test]
fn it_notifies_airbrake() {
    let backtrace = Backtrace::new();

    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: String::from("81bbff95d52f8856c770bb39e827f3f6"),
        ..Default::default()
    });

    let error = "xc".parse::<u32>().err().unwrap();
    let mut params = std::collections::HashMap::new();
    params.insert(String::from("mango"), airbrake::Param::Int32(42));
    params.insert(
        String::from("banana"),
        airbrake::Param::String(String::from("tasty")),
    );

    let notice = notifier
        .build_notice(error)
        .set_backtrace(backtrace)
        .set_params(params);

    let response = notifier.notify(notice).expect("notifier.notify failed");
    assert_eq!(response.status().is_success(), true)
}
