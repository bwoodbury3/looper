//! Looper config parsing crate.
//!
//! Initialize a new project with:
//! ```
//! let project = ProjectConfig::new("my_project.json");
//! ```
//!
//! Blocks typically require a BlockConfig as a configuration input, from which they can read in
//! their runtime configuration. This includes things like input/output streams, Segments for when
//! the block is active, and more. Users configure this in the json "devices" list.
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

use json::JsonValue;

extern crate json;
extern crate log;
extern crate segment;

const SEGMENTS_KEY: &str = "segments";

/// Load a file in as json.
pub fn read_json_file(filename: &str) -> Result<JsonValue, String> {
    let contents = log::unwrap_abort_str!(std::fs::read_to_string(filename));
    let root = log::unwrap_abort_str!(json::parse(&contents));
    return Ok(root);
}

/// Get the path of an audio clip.
pub fn clip_path(clip_name: &str) -> String {
    return format!("assets/clips/{}.wav", clip_name);
}

/// Get the path of an instrument.
pub fn instrument_path(instrument_name: &str) -> String {
    return format!("assets/instruments/{}.json", instrument_name);
}

/// Configuration for a single Block.
pub struct BlockConfig {
    /// The name of the block. This name is completely arbitrary and is only used as an identifier.
    pub name: String,

    /// The type of block. This is used by framework code to determine which Block to instantiate.
    pub block_type: String,

    /// The root json config object.
    root: json::JsonValue,
}

/// Top level config.
pub struct ProjectConfig {
    /// Global configuration parameters.
    pub global_config: json::JsonValue,

    /// The list of blocks.
    pub blocks: Vec<BlockConfig>,
}

impl ProjectConfig {
    /// Initialze a new project config.
    pub fn new(filename: &str) -> Result<ProjectConfig, String> {
        let root = log::unwrap_abort_str!(read_json_file(filename));

        // Read in the global and block configs.
        let global_config = &root["config"];
        let block_config = &root["devices"];
        log::abort_if_msg_str!(!global_config.is_object(), "Missing top-level \"config\" key");
        log::abort_if_msg_str!(!block_config.is_array(), "Missing top-level \"devices\" key");

        // Populate all of the blocks.
        let mut blocks: Vec<BlockConfig> = Vec::new();
        for block in block_config.members() {
            let name = &block["name"];
            let block_type = &block["type"];

            log::abort_if_msg_str!(!name.is_string(), "Block did not contain a valid \"name\"");
            log::abort_if_msg_str!(
                !block_type.is_string(),
                format!("Block \"{}\" not contain a valid \"type\"", name)
            );

            blocks.push(BlockConfig {
                name: name.to_string(),
                block_type: block_type.to_string(),
                root: block.clone(),
            });
        }

        Ok(ProjectConfig {
            global_config: global_config.clone(),
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

impl BlockConfig {
    /// Get a JsonValue from a key. Returns an error if the value is not present.
    /// i.e. obj.is_null() returns true.
    fn get_value(&self, key: &str) -> Result<&json::JsonValue, ()> {
        let value = &self.root[key];
        abort_config!(value.is_null(), self.name, key, "Missing required parameter");
        Ok(value)
    }

    /// Get a string value from config.
    pub fn get_str(&self, key: &str) -> Result<&str, ()> {
        let value = self.get_value(key)?;
        abort_config!(!value.is_string(), self.name, key, "Expected a string value");
        Ok(value.as_str().ok_or(())?)
    }

    /// Get an optional string value from config with a default.
    pub fn get_str_opt<'a>(&'a self, key: &str, default: &'a str) -> Result<&str, ()> {
        let value = &self.root[key];
        if value.is_null() {
            return Ok(default);
        }
        abort_config!(!value.is_string(), self.name, key, "Expected a string value");
        Ok(value.as_str().ok_or(())?)
    }

    /// Get an int value from config.
    pub fn get_i32(&self, key: &str) -> Result<i32, ()> {
        let value = self.get_value(key)?;
        abort_config!(!value.is_number(), self.name, key, "Expected a number");
        Ok(value.as_i32().ok_or(())?)
    }

    /// Get an optional int value from config with a default.
    pub fn get_i32_opt(&self, key: &str, default: &i32) -> Result<i32, ()> {
        let value = &self.root[key];
        if value.is_null() {
            return Ok(*default);
        }
        abort_config!(!value.is_number(), self.name, key, "Expected a number");
        Ok(value.as_i32().ok_or(())?)
    }

    /// Get a float value from config.
    pub fn get_f32(&self, key: &str) -> Result<f32, ()> {
        let value = self.get_value(key)?;
        abort_config!(!value.is_number(), self.name, key, "Expected a number");
        Ok(value.as_f32().ok_or(())?)
    }

    /// Get an optional float value from config with a default.
    pub fn get_f32_opt(&self, key: &str, default: &f32) -> Result<f32, ()> {
        let value = &self.root[key];
        if value.is_null() {
            return Ok(*default);
        }
        abort_config!(!value.is_number(), self.name, key, "Expected a number");
        Ok(value.as_f32().ok_or(())?)
    }

    /// Get a list of output channels.
    pub fn get_str_list(&self, key: &str) -> Result<Vec<&str>, ()> {
        let list = self.get_value(key)?;
        abort_config!(!list.is_array(), self.name, key, "Expected a list of strings");

        let mut str_list: Vec<&str> = Vec::new();
        for member in list.members() {
            abort_config!(!member.is_string(), self.name, key, "Expected a list of strings");
            str_list.push(member.as_str().ok_or(())?);
        }

        Ok(str_list)
    }

    /// Get the segments.
    pub fn get_segments(&self) -> Result<Vec<segment::Segment>, ()> {
        let mut segments: Vec<segment::Segment> = Vec::new();

        // segments is not required. Return empty list if it's missing.
        let list = &self.root[SEGMENTS_KEY];
        if list.is_null() {
            return Ok(segments);
        }

        // Read in the segment config list.
        abort_config!(!list.is_array(), self.name, SEGMENTS_KEY, "Segments must be a list");
        for member in list.members() {
            abort_config!(
                !member.is_object(),
                self.name,
                SEGMENTS_KEY,
                "Each segment must be an object"
            );
            let start = log::unwrap_abort!(member["start"].as_f32().ok_or(()));
            let stop = log::unwrap_abort!(member["stop"].as_f32().ok_or(()));
            let type_str = log::unwrap_abort!(member["type"].as_str().ok_or(()));
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
        let project = ProjectConfig::new("dat/config/valid.json").unwrap();

        // Test the top level config
        let tempo_config = &project.global_config["tempo"];
        assert_eq!(tempo_config["bpm"].as_i32().unwrap(), 101);
        assert_eq!(tempo_config["beats_per_measure"].as_i32().unwrap(), 3);
        assert_eq!(tempo_config["beat_duration"].as_i32().unwrap(), 4);

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
        }
    }

    #[test]
    fn test_missing_devices() {
        // Expect an Err result because the devices config is missing.
        match ProjectConfig::new("dat/config/missing_devices.json") {
            Ok(_) => {
                panic!("Config should have failed to load");
            }
            Err(_) => {}
        };
    }

    #[test]
    fn test_missing_device_type() {
        // Expect an Err result because the "type" field on a device is missing.
        match ProjectConfig::new("dat/config/missing_device_type.json") {
            Ok(_) => {
                panic!("Config should have failed to load");
            }
            Err(_) => {}
        };
    }
}
