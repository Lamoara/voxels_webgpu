use std::{collections::HashMap, hash::RandomState};

use wgpu::ShaderSource;

pub struct ShaderConfig<'a> {
    path: &'a str,
    label: &'a str,
    entry_point: &'a str,
    shader_source: ShaderSource<'a>,

    constants: HashMap<String, f64, RandomState>,
    zero_initialize_workgrouo_memory: bool,

    

}

impl<'a> ShaderConfig<'_> {
    pub fn path(&self) -> &str {
        self.path
    }
    
    pub fn label(&self) -> &str {
        self.label
    }
    
    pub fn entry_point(&self) -> &str {
        self.entry_point
    }
    
    pub fn constants(&self) -> &HashMap<String, f64, RandomState> {
        &self.constants
    }
    
    pub fn zero_initialize_workgrouo_memory(&self) -> bool {
        self.zero_initialize_workgrouo_memory
    }
}