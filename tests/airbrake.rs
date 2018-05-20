extern crate airbrake;

#[test]
fn it_notifies_airbrake() {
    let notifier = airbrake::Notifier::new(airbrake::Config {
        project_id: 113743,
        project_key: String::from("81bbff95d52f8856c770bb39e827f3f6"),
    });

    let error = "xc".parse::<u32>().err().unwrap();
    let response = notifier.notify(error);

    assert_eq!(response.status().is_success(), true)
}
