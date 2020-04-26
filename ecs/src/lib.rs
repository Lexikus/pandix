extern crate legion;

pub use legion::prelude::*;
pub use legion::world::TagSet;
pub use legion::world::TagLayout;
pub use legion::filter::Filter;
pub use legion::filter::ChunksetFilterData;
pub use legion::world::IntoComponentSource;

pub mod prelude {
    pub use crate::legion::prelude::*;
}
