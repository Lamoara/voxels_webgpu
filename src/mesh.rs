use crate::vertex::Vertex;

pub struct Mesh<'a>
{
    label: Option<&'a str>,
    vertices: [Vertex],
}