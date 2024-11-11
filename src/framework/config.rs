extern crate json;
extern crate log;

/// Block configuration.
pub struct BlockConfig {
    pub name: String,
    pub block_type: String,
    pub root: json::JsonValue,
}

/// Top level config.
pub struct ProjectConfig {
    pub global_config: json::JsonValue,
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

impl BlockConfig {
    /// Get a JsonValue from a key. Returns an error if the value is not present.
    /// i.e. obj.is_null() returns true.
    pub fn get_value(&self, key: &str) -> Result<&json::JsonValue, ()> {
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

    /// Get an int value from config.
    pub fn get_int(&self, key: &str) -> Result<i32, ()> {
        let value = self.get_value(key)?;
        abort_config!(!value.is_number(), self.name, key, "Expected a number");
        Ok(value.as_i32().ok_or(())?)
    }

    /// Get a list of output channels
    pub fn get_str_list(&self, key: &str) -> Result<Vec<&str>, ()> {
        let list = self.get_value(key)?;
        if !list.is_array() {
            println!("{}[\"{}\"] must be an array!", self.name, key);
            return Err(());
        }

        let mut str_list: Vec<&str> = Vec::new();
        for member in list.members() {
            if !member.is_string() {
                println!("All list items of {}[\"{}\"] must be strings", self.name, key);
                return Err(());
            }
            str_list.push(member.as_str().ok_or(())?);
        }

        Ok(str_list)
    }
}
