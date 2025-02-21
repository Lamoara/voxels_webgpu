use crate::vertex::Vertex;

pub struct Mesh<'a>
{
    label: Option<&'a str>,
    vertices: Box<[Vertex]>,
}

impl<'a> Mesh<'a> {
    pub fn new(label: Option<&'a str>, vertices: &[Vertex]) -> Mesh<'a>
    {
        Mesh{
            label,
            vertices: vertices.to_vec().into_boxed_slice(),
        }
    }

    pub fn set_vertices(&mut self, vertices: &[Vertex]) {
        self.vertices = vertices.to_vec().into_boxed_slice();
    }
    
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }
}