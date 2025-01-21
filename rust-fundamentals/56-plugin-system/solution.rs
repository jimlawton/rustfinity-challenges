pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self);
}

pub struct PluginManager {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: Vec::<Box<dyn Plugin>>::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        if self.plugins.iter().map(|p| p.name()).any(|name| name == plugin.name()) {
            panic!("Plugin with name '{}' already exists", plugin.name());
        }
        self.plugins.push(plugin);
    }

    pub fn remove_plugin(&mut self, name: &str) -> Option<Box<dyn Plugin>> {
        if self.plugins.iter().map(|p| p.name()).any(|n| n != name) {
            panic!("Plugin with name '{}' does not exist", name);
        }
        let p = self.plugins.iter().position(|p| p.name() == name)?;
        Some(self.plugins.remove(p))
    }

    pub fn execute_all(&self) {
        self.plugins.iter().for_each(|plugin| plugin.execute());
    }
}

