use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

pub use event_id::EventId;

use crate::appmap_definition::event_id::ObjectId;

//region todo objects
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct MetadataObject {}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ExceptionReturnObject {}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct HttpServerRequestCallObject {}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct HttpServerResponseCallObject {}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct HttpClientRequestCallObject {}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct HttpClientResponseCallObject {}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct SqlQueryCallObject {}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct MessageCallObject {}
//endregion

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct AppMapObject {
    pub version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub metadata: Option<MetadataObject>,
    // pub class_map: Vec<CodeObject>,
    #[serde(rename = "classMap")]
    pub class_map: Vec<CodeObjectType>,
    pub events: Vec<EventObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "eventUpdates")]
    pub event_updates: Option<HashMap<u32, EventObject>>,
}
//region events
mod event_id;
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct EventObject {
    //region common
    ///Required unique identifier. Example: 23522.
    pub id: EventId,
    ///Required identifier of the execution thread. Example: 70340688724000.
    pub thread_id: u32,
    //endregion
    #[serde(flatten)]
    pub event: EventObjectType,
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "event")]
#[serde(rename_all = "camelCase")]
pub enum EventObjectType {
    Call(CallObject),
    Return(ReturnObject),
}
//region Return Objects
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ReturnObject {
    ///Required id of the "call" event corresponding to this "return".
    parent_id: u32,
    ///Optional elapsed time in seconds of this function call.
    elapsed: Option<usize>,
    #[serde(flatten)]
    data: ReturnObjectType,
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum ReturnObjectType {
    Normal,
    Function(FunctionReturnObject),
    Exception(ExceptionReturnObject),
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct FunctionReturnObject {
    /// Optional object describing the return value. If present, this value uses parameter object format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub return_value: Option<ParameterObject>,
    /// Optional array of exceptions causing this method to exit. If present, this value uses exception
    /// object format. When an exception is a wrapper for an underlying cause, the cause is the next
    /// exception in the exceptions array.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exceptions: Option<Vec<ExceptionObject>>,
}
//endregion
//region call objects
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct CallObject {
    ///Required name of the class which defines the method. Example: "MyApp::User".
    pub defined_class: String,
    ///Required name of the function which was called in this event. Example: "show".
    pub method_id: String,
    /// Recommended path name of the file which triggered the event. Example: "/src/architecture/lib/appland/local/client.rb".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub path: Option<PathBuf>,
    ///Recommended line number which triggered the event. Example: 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lineno: Option<usize>,
    ///Optional parameter object describing the object on which the function is called. Corresponds to the receiver, self and this concept found in various programming languages.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub receiver: Option<ParameterObject>,
    ///Recommended array of parameter objects describing the function call parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parameters: Option<Vec<ParameterObject>>,
    ///Required flag if the method is class-scoped (static) or instance-scoped. Must be true or false. Example: true.
    #[serde(rename = "static")]
    pub is_static: bool,

    #[serde(flatten)]
    pub type_: CallObjectType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum CallObjectType {
    Normal,
    Function,
    HttpServerRequest(HttpServerRequestCallObject),
    HttpServerResponse(HttpServerResponseCallObject),
    HttpClientRequest(HttpClientRequestCallObject),
    HttpClientResponse(HttpClientResponseCallObject),
    SqlQuery(SqlQueryCallObject),
    Message(MessageCallObject),
}
//endregion

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ParameterObject {
    ///
    ///
    ///Recommended name of the parameter. Example: "login".
    name: Option<String>,
    /// Recommended unique id of the object. Example: 70340693307040
    object_id: Option<ObjectId>,
    ///Required fully qualified class or type name of the object. Example: "MyApp::User".
    class: String,
    ///Required string describing the object. This is not a strict JSON serialization, but rather a display string which is intended for the user. These strings should be trimmed in length to 100 characters. Example: "MyApp user 'alice'"
    value: String,
    /// Recommended number of elements in an array or hash object. Example. "5".
    size: Option<usize>,
    /// Optional schema indicating property names and types of hash and hash-like objects. Each entry is a name, class and optional nested properties or items.
    properties: Option<Vec<PropertiesObject>>,
    /// Optional schema indicating element types of array and array-like objects. Each entry is a class and optional nested properties or items.
    items: Option<Vec<ItemObject>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct PropertiesObject {
    pub name: String,
    pub class: String,
    pub properties: Option<Vec<PropertiesObject>>,
    pub items: Option<Vec<ItemObject>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ItemObject {
    pub class: String,
    pub properties: Option<Vec<PropertiesObject>>,
    pub items: Option<Vec<ItemObject>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ExceptionObject {
    pub class: String,
    pub message: String,
    pub object_id: ObjectId,
    pub path: Option<PathBuf>,
    pub lineno: Option<usize>,
}

//endregion
//region class_map
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct CodeObject {
    pub name: String,
    pub ty: CodeObjectType,
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum CodeObjectType {
    Package(PackageCodeObject),
    Class(ClassCodeObject),
    Function(FunctionCodeObject),
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct PackageCodeObject {
    pub name: String,
    pub children: Option<Vec<CodeObjectType>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ClassCodeObject {
    pub name: String,
    pub children: Option<Vec<CodeObjectType>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct FunctionCodeObject {
    pub name: String,
    ///Recommended File path and line number, separated by a colon. Example: "/Users/alice/src/myapp/lib/myapp/main.rb:5".
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub location: Option<String>,
    ///Required flag if the method is class-scoped (static) or instance-scoped. Must be true or false. Example: true.
    #[serde(rename = "static")]
    pub is_static: bool,
    ///Optional list of arbitrary labels describing the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub labels: Option<Vec<String>>,
    ///Optional documentation comment for the function extracted from the source code.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub comment: Option<String>,
    ///Optional verbatim source code of the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source: Option<String>,
}

//endregion

#[instrument]
pub fn test_sub_mod() {
    info!("test message from test_sub_mod");
}
