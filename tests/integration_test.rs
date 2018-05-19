extern crate airbrake;

use airbrake::{Config, Notifier};

#[test]
fn it_notifies_airbrake() {
    let notifier = Notifier::new(Config {
        project_id: 113743,
        project_key: String::from("81bbff95d52f8856c770bb39e827f3f6"),
    });

    println!("{:?}", notifier.notify(String::from("Bananas!")));
}
