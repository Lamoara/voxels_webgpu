use std::{collections::HashMap, hash::RandomState};

use wgpu::ShaderSource;

pub struct ShaderConfig<'a> {
    pub path: &'a str,
    pub label: &'a str,
    pub entry_point: &'a str,
    pub constants: HashMap<String, f64, RandomState>,
    pub zero_initialize_workgrouo_memory: bool,
}