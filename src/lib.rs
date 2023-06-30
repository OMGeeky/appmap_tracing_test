use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tracing::{Event, Id, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;

use crate::appmap_definition::*;
use crate::extensions::OptionVecExtensions;
use crate::node_functions::*;

pub mod appmap_definition;
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct AppMap {
    #[serde(flatten)]
    pub data: AppMapObject,
    #[serde(skip)]
    next_event_id: u64,
}

#[derive(Debug)]
pub struct AppMapLayer {
    pub test: Mutex<AppMap>,
}

impl AppMapLayer {
    pub fn new() -> Self {
        Self {
            test: Mutex::new(AppMap::new()),
        }
    }
}
impl AppMap {
    pub fn new() -> Self {
        Self {
            data: AppMapObject {
                version: "1.12".to_string(),
                metadata: None,
                class_map: vec![],
                events: vec![],
                event_updates: None,
            },
            next_event_id: 1,
        }
    }
    pub fn get_next_event_id(&mut self) -> u64 {
        let x = self.next_event_id;
        self.next_event_id += 1;
        x
    }

    pub fn add_function_call_event(
        &mut self,
        thread_id: u32,
        class: String,
        method: String,
        path: Option<PathBuf>,
        lineno: Option<usize>,
        is_static: bool,
    ) {
        let id = self.get_next_event_id();
        // let class_name = class.clone();
        // let class_name = class_name.rsplit_once("::").unwrap_or(("", &class)).1;
        self.data.events.push(EventObject {
            id: EventId::from(id),
            thread_id,
            event: EventObjectType::Call(CallObject {
                defined_class: class.to_string(),
                method_id: method.clone(),
                path: path.clone(),
                lineno,
                receiver: None,
                parameters: None,
                is_static,
                type_: CallObjectType::Function,
            }),
        });
        let existing_node = self.find_in_class_map(&class, &method);
        if existing_node.is_none() {
            println!("node not found: {} ; {}", class, method);
            self.add_func_to_hierarchy(
                class,
                method,
                path.map(|x| x.to_str().map(|x| format!("{}:{}", x, lineno.unwrap_or(0))))
                    .flatten(),
            );
        } else {
            println!(
                "node already existing: {} ; {} => {:?}",
                class, method, existing_node
            );
        }
    }
    fn add_func_to_hierarchy(&mut self, class: String, method: String, path: Option<String>) {
        let func = FunctionCodeObject {
            name: method,
            location: path,
            is_static: true,
            labels: None,
            comment: None,
            source: None,
        };
        let func = CodeObjectType::Function(func);

        let class_node = self.find_class_in_class_map_mut(&class);
        if let Some(class_node) = class_node {
            class_node.children.push_or_create(func);
        } else {
            self.add_class_to_hierarchy(&class);
            let class_node = self
                .find_class_in_class_map_mut(&class)
                .expect("We just created this node. It can not be None");
            class_node.children.push_or_create(func);
        }
    }

    fn add_class_to_hierarchy(&mut self, class: &str) {
        println!("class_map: {:?}", self.data.class_map);
        let class_parts = class.split_once("::");
        if let Some((base, name)) = class_parts {
            //class is a subclass. Check if the parent of the class exists already
            let mut parent_class = self.find_class_in_class_map_mut(base);
            if parent_class.is_none() {
                //parent did not exist. Create it!
                self.add_class_to_hierarchy(base);
                parent_class = self.find_class_in_class_map_mut(base);
            }
            let class_node = CodeObjectType::Class(ClassCodeObject {
                name: name.to_string(),
                children: None,
            });
            parent_class
                .expect("Could not find or create the parent class")
                .children
                .push_or_create(class_node);
            println!(
                "added sub class: {} under {} => {:?}",
                name, base, self.data.class_map
            );
            return;
        }
        //could not split so the class should be a top level class
        println!("got add request for top level class: {}", class);

        let top_level_class = self.find_class_in_class_map_mut(class);
        if top_level_class.is_some() {
            return;
        } else {
            let class_node = CodeObjectType::Class(ClassCodeObject {
                name: class.to_string(),
                children: None,
            });

            let classes: &mut Vec<_> = &mut self.data.class_map;
            classes.push(class_node);
            println!("Added top level class: {} => {:?}", class, classes);
        }
    }

    fn find_class_in_class_map(&self, class: &str) -> Option<&ClassCodeObject> {
        for node in self.data.class_map.iter() {
            let class_node = find_class_in_tree(node, class);
            if class_node.is_some() {
                return class_node;
            }
        }
        None
    }
    fn find_in_class_map(&self, class: &str, method: &str) -> Option<&ClassCodeObject> {
        for node in self.data.class_map.iter() {
            let class_node = find_class_in_tree(node, class);
            if let Some(class_node) = class_node {
                if class_node.name == method {
                    return Some(class_node);
                }
            }
        }
        None
    }
    fn find_class_in_class_map_mut(&mut self, class: &str) -> Option<&mut ClassCodeObject> {
        for node in self.data.class_map.iter_mut() {
            let class_node = find_class_in_tree_mut(node, class);
            if class_node.is_some() {
                return class_node;
            }
        }
        None
    }
    fn find_in_class_map_mut(&mut self, class: &str, method: &str) -> Option<&mut CodeObjectType> {
        for node in self.data.class_map.iter_mut() {
            let class_node = find_class_in_tree_mut(node, class);
            if let Some(class_node) = class_node {
                if let Some(children) = class_node.children.as_mut() {
                    for child in children.iter_mut() {
                        let result = is_node_the_searched_function_mut(child, method);
                        if result.is_some() {
                            return result;
                        }
                    }
                }
            }
        }
        None
    }
    pub fn write_to_file(&self) -> Result<(), Box<dyn Error>> {
        let s = serde_json::to_string_pretty(self)?;
        let mut file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(Path::new("maps/tmp/test_1.appmap.json"))?;
        file.write_all(s.as_bytes())?;

        Ok(())
    }
}

impl<S: Subscriber + Debug + for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>>
    Layer<S> for AppMapLayer
{
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        println!("event: {:?}; ctx: {:?}", event, ctx);
    }
    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        println!("on_enter=> id: {:?}; ctx: {:?}", id, ctx);
        let metadata = ctx.metadata(id);
        if let Some(metadata) = metadata {
            let parameters = metadata.fields();
            println!("parameters: {:?}", parameters);
            let x = metadata.module_path().unwrap() == metadata.target();
            println!("some test data: {:?}", x);

            self.test.lock().unwrap().add_function_call_event(
                9999,
                metadata.target().to_string(),
                metadata.name().to_string(),
                metadata.file().map(|f| PathBuf::from(f)),
                metadata.line().map(|x| x as usize),
                true,
            );
            self.test.lock().unwrap().write_to_file().unwrap();
        }
    }
    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        println!("on_close=> id: {:?}; ctx: {:?}", id, ctx);
    }
}
mod extensions;
mod node_functions;
