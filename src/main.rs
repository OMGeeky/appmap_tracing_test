use std::collections::HashMap;
use std::error::Error;
use std::fmt::Error as FmtError;
use std::path::PathBuf;

use appmap_tracing_test::appmap_definition::{
    AppMapObject, CallObject, CallObjectType, ClassCodeObject, CodeObject, CodeObjectType, EventId,
    EventObject, EventObjectType, FunctionCallObject, FunctionCodeObject, MetadataObject,
    PackageCodeObject,
};
use appmap_tracing_test::AppMap;

pub fn main() -> Result<(), Box<dyn Error>> {
    sample_json()?;
    Ok(())
}
fn sample_json() -> Result<(), Box<dyn Error>> {
    let data = AppMap {
        data: AppMapObject {
            metadata: MetadataObject {},
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
                    data: CallObjectType::Function(FunctionCallObject {
                        path: None,
                        lineno: None,
                        receiver: None,
                        parameters: None,
                    }),
                }),
            }],
            version: String::from("1.12"),
            event_updates: None,
        },
    };

    println!("data debug: {:?}", data);
    let data_string = serde_json::to_string_pretty(&data)?;
    println!("data to string: {}", data_string);

    let data_reversed: AppMap = sample_from_str(&data_string)?;
    println!("data_reversed debug: {:?}", data_reversed);
    assert_eq!(data, data_reversed);
    println!("it works!");

    Ok(())
}
fn sample_from_str(s: &str) -> Result<AppMap, Box<dyn Error>> {
    let data_reversed: AppMap = serde_json::from_str(s)?;
    println!("sample_from_str(s): result debug: {:?}", data_reversed);
    Ok(data_reversed)
}
