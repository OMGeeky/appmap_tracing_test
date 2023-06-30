use std::error::Error;
use std::fmt::Error as FmtError;
use std::path::PathBuf;
use tracing::{info, instrument};

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

use appmap_tracing_test::appmap_definition::*;
use appmap_tracing_test::*;

pub fn main() -> Result<(), Box<dyn Error>> {
    init_tracing();

    sample_json()?;
    test_sub_mod();
    Ok(())
}

fn init_tracing() {
    // let stdout_layer = tracing_subscriber::fmt::layer().pretty();
    let app_layer = AppMapLayer::new();

    let subscriber = Registry::default()
        //
        // .with(stdout_layer)
        //
        .with(app_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Unable to set global subscriber");
}

//region AppMapObject
#[instrument]
fn sample_json() -> Result<(), Box<dyn Error>> {
    info!("creating sample object");
    let data = AppMapObject {
        metadata: None,
        class_map: vec![CodeObjectType::Package(PackageCodeObject {
            name: "main pkg".to_string(),
            children: Some(vec![CodeObjectType::Class(ClassCodeObject {
                name: "main cls".to_string(),
                children: Some(vec![CodeObjectType::Function(FunctionCodeObject {
                    name: "sample_json".to_string(),
                    location: Some(format!(
                        "{}:{}",
                        PathBuf::from("src/main.rs").to_str().ok_or(FmtError)?,
                        14
                    )),
                    is_static: true,
                    labels: Some(vec!["security".to_string()]),
                    comment: None,
                    source: None,
                })]),
            })]),
        })],
        events: vec![EventObject {
            id: EventId::from(1),
            thread_id: 9999,
            event: EventObjectType::Call(CallObject {
                defined_class: "main".to_string(),
                method_id: "sample_json".to_string(),
                path: Some(PathBuf::from("src/main.rs")),
                lineno: Some(14),
                receiver: None,
                parameters: None,
                is_static: true,
                type_: CallObjectType::Function,
            }),
        }],
        version: String::from("1.12"),
        event_updates: None,
    };

    info!("data debug: {:?}", data);
    let data_string = serde_json::to_string_pretty(&data)?;
    info!("data to string: {}", data_string);
    // println!("data to string: {}", data_string);

    let data_reversed: AppMapObject = sample_from_str(&data_string)?;
    info!("data_reversed debug: {:?}", data_reversed);
    assert_eq!(data, data_reversed);
    info!("it works!");
    // println!("it works!");

    Ok(())
}
#[instrument]
fn sample_from_str(s: &str) -> Result<AppMapObject, Box<dyn Error>> {
    let data_reversed: AppMapObject = serde_json::from_str(s)?;
    info!("sample_from_str(s): result debug: {:?}", data_reversed);
    Ok(data_reversed)
}
//endregion
