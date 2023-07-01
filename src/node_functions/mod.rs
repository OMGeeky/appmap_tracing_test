use crate::appmap_definition::*;

pub fn find_class_in_tree<'a>(
    node: &'a CodeObjectType,
    class: &str,
) -> Option<&'a ClassCodeObject> {
    let children = match node {
        CodeObjectType::Class(c) => {
            if class.ends_with(&c.name) {
                return Some(c);
            }
            if class.starts_with(&c.name) {
                c.children.as_ref()
            } else {
                None
            }
        }
        _ => None,
    };
    if let Some(children) = children {
        for x in children.iter() {
            let child_found = find_class_in_tree(x, class);
            if child_found.is_some() {
                return child_found;
            }
        }
    }
    return None;
}
pub fn is_node_the_searched_function<'a>(
    node: &'a CodeObjectType,
    method: &str,
) -> Option<&'a CodeObjectType> {
    match node {
        CodeObjectType::Function(f) => {
            if f.name == method {
                return Some(node);
            }
            None
        }
        _ => None,
    }
}
pub fn find_class_in_tree_mut<'a>(
    node: &'a mut CodeObjectType,
    class: &str,
) -> Option<&'a mut ClassCodeObject> {
    // println!("trying to find class: {} in node: {:?}", class, node);
    let children = match node {
        CodeObjectType::Class(c) => {
            if class.ends_with(&c.name) {
                return Some(c);
            }
            if class.starts_with(&c.name) {
                c.children.as_mut()
            } else {
                None
            }
        }
        _ => None,
    };
    if let Some(children) = children {
        for x in children.iter_mut() {
            let child_found = find_class_in_tree_mut(x, class);
            if child_found.is_some() {
                return child_found;
            }
        }
    }
    return None;
}
pub fn is_node_the_searched_function_mut<'a>(
    node: &'a mut CodeObjectType,
    method: &str,
) -> Option<&'a mut CodeObjectType> {
    match node {
        CodeObjectType::Function(f) => {
            if f.name == method {
                return Some(node);
            }
            None
        }
        _ => None,
    }
}
