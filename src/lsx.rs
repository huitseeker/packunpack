use anyhow::Result;
use quick_xml::{Reader, Writer, events::{Event, BytesEnd, BytesStart, BytesText}};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

use crate::resource::{Resource, Metadata, Region, Node, NodeAttribute, AttributeType, AttributeValue};

pub fn write_lsx<P: AsRef<Path>>(resource: &Resource, path: P) -> Result<()> {
    let file = File::create(path)?;
    let mut writer = Writer::new_with_indent(BufWriter::new(file), b' ', 2);
    
    // Write XML declaration
    writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new(
        "1.0", Some("utf-8"), None
    )))?;
    
    // Write root save element
    let mut save_elem = BytesStart::new("save");
    writer.write_event(Event::Start(save_elem.clone()))?;
    
    // Write version element
    let mut version_elem = BytesStart::new("version");
    version_elem.push_attribute(("major", resource.metadata.major_version.to_string().as_str()));
    version_elem.push_attribute(("minor", resource.metadata.minor_version.to_string().as_str()));
    version_elem.push_attribute(("revision", resource.metadata.revision.to_string().as_str()));
    version_elem.push_attribute(("build", resource.metadata.build_number.to_string().as_str()));
    writer.write_event(Event::Empty(version_elem))?;
    
    // Write regions
    for (region_name, region) in &resource.regions {
        write_region(&mut writer, region_name, region)?;
    }
    
    // Close save element
    writer.write_event(Event::End(BytesEnd::new("save")))?;
    
    Ok(())
}

fn write_region<W: Write>(writer: &mut Writer<W>, region_name: &str, region: &Region) -> Result<()> {
    let mut region_elem = BytesStart::new("region");
    region_elem.push_attribute(("id", region_name));
    writer.write_event(Event::Start(region_elem.clone()))?;
    
    // Write all nodes in the region
    for node in &region.nodes {
        write_node(writer, node)?;
    }
    
    writer.write_event(Event::End(BytesEnd::new("region")))?;
    Ok(())
}

fn write_node<W: Write>(writer: &mut Writer<W>, node: &Node) -> Result<()> {
    let mut node_elem = BytesStart::new("node");
    node_elem.push_attribute(("id", node.id.as_str()));
    writer.write_event(Event::Start(node_elem.clone()))?;
    
    // Write attributes
    for (attr_name, attr) in &node.attributes {
        write_attribute(writer, attr_name, attr)?;
    }
    
    // Write children if any
    if !node.children.is_empty() {
        let children_elem = BytesStart::new("children");
        writer.write_event(Event::Start(children_elem.clone()))?;
        
        for child in &node.children {
            write_node(writer, child)?;
        }
        
        writer.write_event(Event::End(BytesEnd::new("children")))?;
    }
    
    writer.write_event(Event::End(BytesEnd::new("node")))?;
    Ok(())
}

fn write_attribute<W: Write>(writer: &mut Writer<W>, attr_name: &str, attr: &NodeAttribute) -> Result<()> {
    let mut attr_elem = BytesStart::new("attribute");
    attr_elem.push_attribute(("id", attr_name));
    attr_elem.push_attribute(("type", attr.attribute_type.as_str()));
    attr_elem.push_attribute(("value", attr.value.to_string().as_str()));
    writer.write_event(Event::Empty(attr_elem))?;
    Ok(())
}

pub fn read_lsx<P: AsRef<Path>>(path: P) -> Result<Resource> {
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(BufReader::new(file));
    reader.trim_text(true);
    
    let mut resource = Resource {
        metadata: Metadata {
            major_version: 1,
            minor_version: 0,
            revision: 0,
            build_number: 0,
        },
        regions: HashMap::new(),
    };
    
    let mut buf = Vec::new();
    let mut node_stack: Vec<Node> = Vec::new();
    let mut current_region: Option<Region> = None;
    
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"save" => {
                        // Root element, continue
                    },
                    b"version" => {
                        // Read version attributes
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.as_ref() {
                                b"major" => resource.metadata.major_version = parse_attr_value(&attr.value)?,
                                b"minor" => resource.metadata.minor_version = parse_attr_value(&attr.value)?,
                                b"revision" => resource.metadata.revision = parse_attr_value(&attr.value)?,
                                b"build" => resource.metadata.build_number = parse_attr_value(&attr.value)?,
                                _ => {}
                            }
                        }
                    },
                    b"region" => {
                        let mut region_id = String::new();
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.as_ref() == b"id" {
                                region_id = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                        current_region = Some(Region {
                            name: region_id,
                            nodes: Vec::new(),
                        });
                    },
                    b"node" => {
                        let mut node_id = String::new();
                        for attr in e.attributes() {
                            let attr = attr?;
                            if attr.key.as_ref() == b"id" {
                                node_id = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                        let node = Node {
                            id: node_id,
                            name: None,
                            parent: None,
                            attributes: HashMap::new(),
                            children: Vec::new(),
                        };
                        node_stack.push(node);
                    },
                    b"children" => {
                        // Children container, no action needed
                    },
                    _ => {}
                }
            },
            Event::Empty(e) => {
                match e.name().as_ref() {
                    b"attribute" => {
                        let mut attr_id = String::new();
                        let mut attr_type = String::new();
                        let mut attr_value = String::new();
                        
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.as_ref() {
                                b"id" => attr_id = String::from_utf8_lossy(&attr.value).to_string(),
                                b"type" => attr_type = String::from_utf8_lossy(&attr.value).to_string(),
                                b"value" => attr_value = String::from_utf8_lossy(&attr.value).to_string(),
                                _ => {}
                            }
                        }
                        
                        if let Some(current_node) = node_stack.last_mut() {
                            if let Some(parsed_type) = AttributeType::from_str(&attr_type) {
                                let parsed_value = AttributeValue::from_string(&parsed_type, &attr_value)?;
                                current_node.attributes.insert(attr_id, NodeAttribute {
                                    attribute_type: parsed_type,
                                    value: parsed_value,
                                });
                            }
                        }
                    },
                    b"version" => {
                        // Handle empty version element
                        for attr in e.attributes() {
                            let attr = attr?;
                            match attr.key.as_ref() {
                                b"major" => resource.metadata.major_version = parse_attr_value(&attr.value)?,
                                b"minor" => resource.metadata.minor_version = parse_attr_value(&attr.value)?,
                                b"revision" => resource.metadata.revision = parse_attr_value(&attr.value)?,
                                b"build" => resource.metadata.build_number = parse_attr_value(&attr.value)?,
                                _ => {}
                            }
                        }
                    },
                    _ => {}
                }
            },
            Event::End(e) => {
                match e.name().as_ref() {
                    b"node" => {
                        if let Some(completed_node) = node_stack.pop() {
                            if let Some(parent_node) = node_stack.last_mut() {
                                // Add as child to parent node
                                parent_node.children.push(completed_node);
                            } else if let Some(region) = &mut current_region {
                                // Add as root node to current region
                                region.nodes.push(completed_node);
                            }
                        }
                    },
                    b"region" => {
                        if let Some(region) = current_region.take() {
                            resource.regions.insert(region.name.clone(), region);
                        }
                    },
                    b"children" => {
                        // End of children container, no action needed
                    },
                    b"save" => {
                        // End of document
                        break;
                    },
                    _ => {}
                }
            },
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    
    Ok(resource)
}

fn parse_attr_value(value: &[u8]) -> Result<u32> {
    let value_str = String::from_utf8_lossy(value);
    Ok(value_str.parse()?)
}