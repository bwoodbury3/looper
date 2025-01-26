//! Looper config parsing crate.
//!
//! Initialize a new project with:
//! ```
//! let project = ProjectConfig::new("my_project.yaml");
//! ```
//!
//! Blocks typically require a BlockConfig as a configuration input, from which they can read in
//! their runtime configuration. This includes things like input/output streams, Segments for when
//! the block is active, and more. Users configure this in the yaml "devices" list.
//!
//! BlockConfig uses generic type-safe key/value accessors to retrieve config parameters. Blocks
//! should read in all config values in their initialization function (::new) like so:
//! ```
//! extern crate config;
//!
//! impl MyBlock {
//!     pub fn new(
//!         config: &config::BlockConfig,
//!         stream_catalog: &mut stream::StreamCatalog
//!     ) -> Result<Self, ()> {
//!         let int_param = config.get_int("int_param")?;
//!         let str_param = config.get_str("str_param")?;
//!         ...
//!     }
//! }
//! ```
//!
//! Block configuration supports the following generic parameters that can be configured by users.
//!  - `str`
//!  - `i32`
//!  - `f32`
//!  - `Vec<str>`
//!  - `Vec<segment::Segment>`

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use yaml_rust::{Yaml, YamlLoader};

extern crate log;
extern crate segment;
extern crate yaml_rust;

const SEGMENTS_KEY: &str = "segments";

/// Load a file in as yaml.
pub fn read_yaml_file(filename: &str) -> Result<Vec<Yaml>, String> {
    let contents = log::unwrap_abort_str!(std::fs::read_to_string(filename));
    let root = log::unwrap_abort_str!(YamlLoader::load_from_str(&contents));
    return Ok(root);
}

/// Get the path of an audio clip.
pub fn clip_path(clip_name: &str) -> String {
    return format!("assets/clips/{}.wav", clip_name);
}

/// Get the path of an instrument.
pub fn instrument_path(instrument_name: &str) -> String {
    return format!("assets/instruments/{}.yaml", instrument_name);
}

/// Named variables defined by the user.
struct NamedVariables {
    /// Integer variables.
    pub vars_i32: HashMap<String, i32>,

    /// Floating variables.
    pub vars_f32: HashMap<String, f32>,

    /// String variables.
    pub vars_str: HashMap<String, String>,
}

/// Configuration for a single Block.
pub struct BlockConfig {
    /// The name of the block. This name is completely arbitrary and is only used as an identifier.
    pub name: String,

    /// The type of block. This is used by framework code to determine which Block to instantiate.
    pub block_type: String,

    /// The root yaml config object.
    root: Yaml,

    /// User-defined variables.
    variables: Rc<RefCell<NamedVariables>>,
}

/// Top level config.
pub struct ProjectConfig {
    /// Global configuration parameters.
    pub tempo_config: Yaml,

    /// The start measure.
    pub start_measure: f32,

    /// The stop measure.
    pub stop_measure: f32,

    /// The list of blocks.
    pub blocks: Vec<BlockConfig>,
}

// Parse a yaml object as an i32.
fn yaml_as_i32(obj: &Yaml) -> Result<i32, ()> {
    let result = match obj {
        Yaml::Integer(i) => *i as i32,
        _ => {
            log::abort_msg!("Expected an integer value");
        }
    };
    Ok(result)
}

// Parse a yaml object as an i32.
fn yaml_as_i32_opt(obj: &Yaml, default: &i32) -> i32 {
    match obj {
        Yaml::Integer(i) => *i as i32,
        _ => *default,
    }
}

// Parse a yaml object as an f32. Supports casting if the object is an int.
fn yaml_as_f32(obj: &Yaml) -> Result<f32, ()> {
    let result = match obj {
        Yaml::Real(_) => obj.as_f64().unwrap() as f32,
        Yaml::Integer(_) => obj.as_i64().unwrap() as f32,
        _ => {
            log::abort_msg!("Expected a number value");
        }
    };
    Ok(result)
}

// Parse a yaml object as an f32. Supports casting if the object is an int.
fn yaml_as_f32_opt(obj: &Yaml, default: &f32) -> f32 {
    match obj {
        Yaml::Real(_) => obj.as_f64().unwrap() as f32,
        Yaml::Integer(_) => obj.as_i64().unwrap() as f32,
        _ => *default,
    }
}

impl ProjectConfig {
    /// Initialze a new project config.
    pub fn new(filename: &str) -> Result<ProjectConfig, String> {
        let root = &log::unwrap_abort_str!(read_yaml_file(filename))[0];

        // Read in the global configs.
        let global_config = &root["config"];
        log::abort_if_msg_str!(global_config.is_badvalue(), "Missing top-level \"config\" key");

        let start_measure = yaml_as_f32_opt(&global_config["start_measure"], &0f32);
        let stop_measure = yaml_as_f32_opt(&global_config["stop_measure"], &-1f32);
        println!("Start measure: {}", start_measure);
        println!("Stop measure: {}", stop_measure);

        // Load all of the variables.
        let mut vars_i32: HashMap<String, i32> = HashMap::new();
        let mut vars_f32: HashMap<String, f32> = HashMap::new();
        let mut vars_str: HashMap<String, String> = HashMap::new();
        match root["variables"].as_hash() {
            Some(var_config) => {
                for (yk, value) in var_config.iter() {
                    let key = log::opt_abort_str!(yk.as_str(), "Variable key must be a string");
                    match value {
                        Yaml::Integer(v) => {
                            vars_i32.insert(key.to_owned(), *v as i32);
                            vars_f32.insert(key.to_owned(), *v as f32);
                        }
                        Yaml::Real(_) => {
                            vars_f32.insert(key.to_owned(), value.as_f64().unwrap() as f32);
                        }
                        Yaml::String(s) => {
                            vars_str.insert(key.to_owned(), s.to_owned());
                        }
                        _ => {
                            return Err(format!("Unsupport variable type for \"{}\"", key));
                        }
                    }
                }
            }
            None => {}
        };
        let variables = Rc::new(RefCell::new(NamedVariables {
            vars_i32: vars_i32,
            vars_f32: vars_f32,
            vars_str: vars_str,
        }));

        // Load all of the blocks.
        let block_config = match root["devices"].as_vec() {
            Some(v) => v,
            None => {
                return Err("\"devices\" must be a list".to_owned());
            }
        };
        let mut blocks: Vec<BlockConfig> = Vec::new();
        for block in block_config {
            let name = log::opt_abort_str!(
                block["name"].as_str(),
                "Block did not contain a valid \"name\""
            );
            let block_type = log::opt_abort_str!(
                block["type"].as_str(),
                format!("Block \"{}\" did not contain a valid \"type\"", name)
            );

            blocks.push(BlockConfig {
                name: name.to_owned(),
                block_type: block_type.to_owned(),
                root: block.clone(),
                variables: variables.clone(),
            });
        }

        Ok(ProjectConfig {
            tempo_config: global_config["tempo"].clone(),
            start_measure: start_measure as f32,
            stop_measure: stop_measure as f32,
            blocks: blocks,
        })
    }
}

/// Shorthand for asserting configuration values are valid with context.
macro_rules! abort_config {
    ( $e:expr, $name:expr, $key:expr, $msg:expr ) => {
        log::abort_if_msg!($e, format!("device=\"{}\" -> key=\"{}\": {}", $name, $key, $msg));
    };
}

/// Shorthand for asserting configuration values are valid with context.
macro_rules! unwrap_config {
    ( $e:expr, $name:expr, $key:expr, $msg:expr ) => {
        log::opt_abort_msg!($e, format!("device=\"{}\" -> key=\"{}\": {}", $name, $key, $msg))
    };
}

impl BlockConfig {
    /// Get a Yaml value from a key. Returns an error if the value is not present.
    /// i.e. obj.is_badvalue() returns true.
    pub fn get_value(&self, key: &str) -> Result<&Yaml, ()> {
        let value = &self.root[key];
        abort_config!(value.is_badvalue(), self.name, key, "Missing required parameter");
        Ok(value)
    }

    /// Get a boolean value from config.
    pub fn get_bool(&self, key: &str) -> Result<bool, ()> {
        let value = self.get_value(key)?;
        Ok(unwrap_config!(value.as_bool(), self.name, key, "Expected a boolean value"))
    }

    /// Get an optional bool value from config with a default.
    pub fn get_bool_opt<'a>(&'a self, key: &str, default: bool) -> Result<bool, ()> {
        let value = &self.root[key];
        if value.is_badvalue() {
            return Ok(default);
        }
        Ok(unwrap_config!(value.as_bool(), self.name, key, "Expected a boolean value"))
    }

    /// Get a string value from config.
    pub fn get_str(&self, key: &str) -> Result<&str, ()> {
        let value = self.get_value(key)?;
        Ok(unwrap_config!(value.as_str(), self.name, key, "Expected a string value"))
    }

    /// Get an optional string value from config with a default.
    pub fn get_str_opt<'a>(&'a self, key: &str, default: &'a str) -> Result<&str, ()> {
        let value = &self.root[key];
        if value.is_badvalue() {
            return Ok(default);
        }
        Ok(unwrap_config!(value.as_str(), self.name, key, "Expected a string value"))
    }

    /// Get an int value from config.
    pub fn get_i32(&self, key: &str) -> Result<i32, ()> {
        let value = self.get_value(key)?;
        yaml_as_i32(value)
    }

    /// Get an optional int value from config with a default.
    pub fn get_i32_opt(&self, key: &str, default: &i32) -> Result<i32, ()> {
        let value = &self.root[key];
        Ok(yaml_as_i32_opt(value, default))
    }

    /// Get a float value from config.
    pub fn get_f32(&self, key: &str) -> Result<f32, ()> {
        let value = self.get_value(key)?;
        yaml_as_f32(value)
    }

    /// Get an optional float value from config with a default.
    pub fn get_f32_opt(&self, key: &str, default: &f32) -> Result<f32, ()> {
        let value = &self.root[key];
        Ok(yaml_as_f32_opt(value, default))
    }

    /// Get a list of output channels.
    pub fn get_str_list(&self, key: &str) -> Result<Vec<&str>, ()> {
        let value = self.get_value(key)?;
        let list_vec = unwrap_config!(value.as_vec(), self.name, key, "Expected a list");

        let mut str_list: Vec<&str> = Vec::new();
        for member in list_vec {
            let val = unwrap_config!(member.as_str(), self.name, key, "Expected a string value");
            str_list.push(val);
        }

        Ok(str_list)
    }

    /// Get the segments.
    pub fn get_segments(&self) -> Result<Vec<segment::Segment>, ()> {
        let mut segments: Vec<segment::Segment> = Vec::new();

        // segments is not required. Return empty list if it's missing.
        let list = &self.root[SEGMENTS_KEY];
        if list.is_badvalue() {
            return Ok(segments);
        }

        // Read in the segment config list.
        let list_vec = unwrap_config!(list.as_vec(), self.name, "segments", "Must be a list");
        for member in list_vec {
            abort_config!(
                member.is_badvalue(),
                self.name,
                SEGMENTS_KEY,
                "Each segment must be an object"
            );
            let start = log::unwrap_abort!(yaml_as_f32(&member["start"]));
            let stop = log::unwrap_abort!(yaml_as_f32(&member["stop"]));
            let type_str = log::opt_abort!(member["type"].as_str());
            let name = match member["name"].as_str() {
                Some(s) => Some(s.to_owned()),
                None => None,
            };

            abort_config!(
                start > stop,
                self.name,
                SEGMENTS_KEY,
                "Segment start must be < segment stop"
            );

            segments.push(segment::Segment {
                start: start,
                stop: stop,
                segment_type: segment::SegmentType::from(type_str),
                name: name,
            })
        }

        Ok(segments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_config() {
        // This should load with no problems.
        let project = ProjectConfig::new("dat/config/valid.yaml").unwrap();

        // Test the top level config
        let tempo_config = &project.tempo_config;
        assert_eq!(tempo_config["bpm"].as_i64().unwrap(), 101);
        assert_eq!(tempo_config["beats_per_measure"].as_i64().unwrap(), 3);
        assert_eq!(tempo_config["beat_duration"].as_i64().unwrap(), 4);

        assert_eq!(project.start_measure, 0.0);
        assert_eq!(project.stop_measure, 20.0);

        // Test the block getters.
        let blocks = &project.blocks;
        {
            let block = &blocks[0];
            assert_eq!(block.name, "drums");
            assert_eq!(block.block_type, "VirtualInstrument");
            assert_eq!(block.get_str("instrument").unwrap(), "drums1");
            assert_eq!(block.get_f32("volume").unwrap(), 0.2);
            assert_eq!(block.get_f32_opt("optional_param", &2.3).unwrap(), 2.3);

            let segments = block.get_segments().unwrap();
            let segment0 = &segments[0];
            assert_eq!(segment0.start, 1.0);
            assert_eq!(segment0.stop, 2.0);
            assert!(segment0.segment_type == segment::SegmentType::Input);
            let segment1 = &segments[1];
            assert_eq!(segment1.start, 3.0);
            assert_eq!(segment1.stop, 4.0);
            assert!(segment1.segment_type == segment::SegmentType::Output);
        }
        {
            let block = &blocks[1];
            assert_eq!(block.name, "combiner");
            assert_eq!(block.block_type, "Combiner");
            assert_eq!(block.get_str_list("param_str_list").unwrap(), vec!["val1", "val2"]);
            assert_eq!(block.get_i32("param_i32").unwrap(), 12);
            assert_eq!(block.get_i32_opt("opt_param_i32", &13).unwrap(), 13);
            assert_eq!(block.get_bool("param_bool").unwrap(), true);
            assert_eq!(block.get_bool_opt("opt_param_bool", false).unwrap(), false);
        }
    }

    #[test]
    fn test_missing_devices() {
        // Expect an Err result because the devices config is missing.
        match ProjectConfig::new("dat/config/missing_devices.yaml") {
            Ok(_) => {
                panic!("Config should have failed to load");
            }
            Err(_) => {}
        };
    }

    #[test]
    fn test_missing_device_type() {
        // Expect an Err result because the "type" field on a device is missing.
        match ProjectConfig::new("dat/config/missing_device_type.yaml") {
            Ok(_) => {
                panic!("Config should have failed to load");
            }
            Err(_) => {}
        };
    }
}
