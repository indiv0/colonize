use std::u16;

pub use glium::Vertex as VertexType;

pub struct Mesh<T>
    where T: VertexType,
{
    pub vertices: Vec<T>,
    pub indices: Vec<u16>,
}

impl<T> Mesh<T>
    where T: VertexType,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, vertices: Vec<T>, faces: Vec<[u16; 3]>) {
        assert!(self.vertices.len() + vertices.len() < u16::MAX as usize);
        let offset = self.vertices.len() as u16;
        for v in vertices.into_iter() {
            self.vertices.push(v);
        }

        for face in faces.into_iter() {
            for i in face.into_iter() {
                self.indices.push(i + offset);
            }
        }
    }
}

impl<T> Default for Mesh<T>
    where T: VertexType,
{
    fn default() -> Self {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}

pub trait ToMesh<T>
    where T: VertexType,
{
    fn mesh(self) -> Mesh<T>;
}
