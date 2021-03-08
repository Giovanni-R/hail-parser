use serde::{de::DeserializeOwned, Deserialize};

use parser::types::metadata::{ComponentMetadata, MatrixMetadata, TableMetadata};

/// This a variant of [Matrix](parser::Matrix) that serde to deserialize the data
/// directly into Rust data structures, with no indirection.
#[derive(Clone, Debug)]
pub struct SerdeMatrix<G, C, R, E> {
    pub globals: SerdeComponent<G>,
    pub cols: SerdeComponent<C>,
    pub rows: SerdeComponent<R>,
    pub entries: SerdeComponent<E>,
    pub metadata: MatrixMetadata,
}

/// This a variant of [Table](parser::Table) that serde to deserialize the data
/// directly into Rust data structures, with no indirection.
#[derive(Clone, Debug)]
pub struct SerdeTable<G, R> {
    pub globals: SerdeComponent<G>,
    pub rows: SerdeComponent<R>,
    pub metadata: TableMetadata,
}

/// This a variant of [Component](parser::Component) that serde to deserialize the data
/// directly into Rust data structures, with no indirection.
#[derive(Clone, Debug)]
pub struct SerdeComponent<R> {
    pub data: Vec<Vec<R>>,
    pub metadata: ComponentMetadata,
    // index: Option<Index>,
}

#[derive(Debug, Deserialize)]
pub struct Call(pub u32);

#[derive(Debug, Deserialize)]
pub struct Locus(pub String, pub u32);

#[derive(Debug, Deserialize)]
pub struct Interval<T> {
    pub start: T,
    pub end: T,
    pub includes_start: bool,
    pub includes_end: bool,
}

#[derive(Debug)]
pub struct NDArray<T: DeserializeOwned, const N: usize>(pub ndarray::ArrayD<T>);
