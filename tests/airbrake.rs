extern crate airbrake;

#[test]
fn it_notifies_airbrake_without_params() {
    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: String::from("81bbff95d52f8856c770bb39e827f3f6"),
    });

    let error = "xc".parse::<u32>().err().unwrap();
    let response = notifier.notify(error, None);
    assert_eq!(response.status().is_success(), true)
}

#[test]
fn it_notifies_airbrake_with_some_params() {
    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: String::from("81bbff95d52f8856c770bb39e827f3f6"),
    });

    let mut params = std::collections::HashMap::new();
    params.insert(String::from("mango"), airbrake::notice::Param::Int32(42));
    params.insert(
        String::from("banana"),
        airbrake::notice::Param::String(String::from("tasty")),
    );

    let error = "xc".parse::<f64>().err().unwrap();
    let response = notifier.notify(error, Some(params));
    assert_eq!(response.status().is_success(), true)
}
