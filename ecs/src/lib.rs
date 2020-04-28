extern crate legion;

pub use legion::filter::ChunksetFilterData;
pub use legion::filter::Filter;
pub use legion::prelude::*;
pub use legion::systems::resource::Resource;
pub use legion::world::IntoComponentSource;
pub use legion::world::TagLayout;
pub use legion::world::TagSet;

pub mod prelude {
    pub use crate::legion::prelude::*;
}
