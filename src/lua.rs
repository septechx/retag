use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use mlua::prelude::*;

use crate::trackdata::TrackData;

pub struct LuaEngine {
    lua: Lua,
    callbacks: Arc<Mutex<Vec<LuaRegistryKey>>>,
}

impl LuaEngine {
    pub fn new() -> LuaResult<Self> {
        let lua = Lua::new();
        let callbacks: Arc<Mutex<Vec<LuaRegistryKey>>> = Arc::new(Mutex::new(Vec::new()));

        let cb_clone = callbacks.clone();

        let register = lua.create_function(move |lua, func: LuaFunction| {
            let key = lua.create_registry_value(func)?;
            cb_clone.lock().unwrap().push(key);
            Ok(())
        })?;

        let api = lua.create_table()?;
        api.set("register", register)?;
        lua.globals().set("retag", api)?;

        Ok(Self { lua, callbacks })
    }

    pub fn load_script(&self, path: &Path) -> Result<()> {
        let script = std::fs::read_to_string(path)?;
        self.lua.load(&script).exec()?;
        Ok(())
    }

    pub fn run_callbacks(&self, data: &TrackData) -> Result<()> {
        let keys = self.callbacks.lock().unwrap();

        let value = self.lua.to_value(data)?;

        for key in keys.iter() {
            let func: LuaFunction = self.lua.registry_value(key)?;
            func.call::<()>(value.clone())?;
        }

        Ok(())
    }
}
