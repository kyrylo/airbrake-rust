extern crate airbrake;

#[test]
fn it_notifies_airbrake() {
    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: String::from("81bbff95d52f8856c770bb39e827f3f6"),
    });

    let error1 = "xc".parse::<u32>().err().unwrap();
    let response = notifier.notify(error1, None);
    assert_eq!(response.status().is_success(), true);

    let mut params = std::collections::HashMap::new();
    params.insert(String::from("mango"), airbrake::Param::Int32(42));
    params.insert(
        String::from("banana"),
        airbrake::Param::String(String::from("tasty")),
    );

    let error2 = "xc".parse::<f64>().err().unwrap();
    let response = notifier.notify(error2, Some(params));
    assert_eq!(response.status().is_success(), true)
}
