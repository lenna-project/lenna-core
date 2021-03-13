use crate::core::processor::Processor;
use crate::core::pool::Pool;

#[derive(Copy, Clone)]
pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

pub trait PluginRegistrar {
    fn add_plugin(&mut self, plugin: Box<dyn Processor>);
}

impl PluginRegistrar for Pool {
    fn add_plugin(&mut self, plugin: Box<dyn Processor>){
        self.add(plugin)
    }
}
