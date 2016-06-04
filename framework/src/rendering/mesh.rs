use std::u16;

use glium;
use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};

pub use glium::VertexBuffer;
pub use glium::Vertex as VertexType;

/// Vertex index data type.
pub type Index = u16;
/// A list of indices loaded in the GPU's memory.
pub type IndexBuffer = glium::IndexBuffer<Index>;

pub const INDEX_TYPE: PrimitiveType = PrimitiveType::TrianglesList;

// TODO: rename this enum.
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// Mesh type, containing a vertex buffer and an index buffer.
pub struct Mesh<T>
    where T: VertexType,
{
    pub vertices: VertexBuffer<T>,
    // Using `Result` here is a bit of a hack to allow use to use one of two
    // types for the index storage.
    pub indices: Either<IndexBuffer, NoIndices>,
}

impl<T> Mesh<T>
    where T: VertexType,
{
    pub fn new<F>(facade: &F, vertices: &[T]) -> Self
        where F: Facade,
    {
        Mesh {
            vertices: VertexBuffer::new(facade, vertices).unwrap(),
            indices: Either::Right(NoIndices(INDEX_TYPE)),
        }
    }

    pub fn with_indices<F>(
        facade: &F,
        vertices: &[T],
        indices: &[Index],
    ) -> Self
        where F: Facade,
    {
        Mesh {
            vertices: VertexBuffer::new(facade, vertices).unwrap(),
            indices: Either::Left(IndexBuffer::new(facade, INDEX_TYPE, indices).unwrap()),
        }
    }
}

pub trait ToMesh<T>
    where T: VertexType,
{
    fn mesh(self) -> Mesh<T>;
}
