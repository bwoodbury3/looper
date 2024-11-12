//! Looper config parsing crate.
//!
//! Initialize a new project with:
//! ```
//! let project = ProjectConfig::new("my_project.json");
//! ```
//!
//! Blocks typically require a BlockConfig as a configuration input, from which they can read in
//! their runtime configuration. This includes things like input/output streams, Segments for when
//! the block is active, and more.
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

extern crate json;
extern crate log;

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
        let contents = log::unwrap_abort_str!(std::fs::read_to_string(filename));
        let root = log::unwrap_abort_str!(json::parse(&contents));

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

/// Get the path of an audio clip.
pub fn clip_path(clip_name: &str) -> String {
    return format!("assets/clips/{}.wav", clip_name);
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

    /// Get an optional string value from config with a default.
    pub fn get_i32_opt(&self, key: &str, default: &i32) -> Result<i32, ()> {
        let value = &self.root[key];
        if value.is_null() {
            return Ok(*default);
        }
        abort_config!(!value.is_number(), self.name, key, "Expected a number");
        Ok(value.as_i32().ok_or(())?)
    }

    /// Get a list of output channels
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
}
