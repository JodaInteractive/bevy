use bevy_reflect_derive::impl_type_path;

#[cfg(feature = "functions")]
use crate::{from_reflect::FromReflect, type_path::TypePath, type_registry::GetTypeRegistration};
#[cfg(feature = "functions")]
use core::hash::{BuildHasher, Hash};

impl_reflect_for_entity_index_set!(::bevy_ecs::entity::index_map::EntityIndexSet<V>);
impl_type_path!(::bevy_ecs::entity::index_map::EntityIndexSet<V>);
#[cfg(feature = "functions")]
crate::func::macros::impl_function_traits!(::bevy_ecs::entity::index_map::EntityIndexSet<V>;
    <
        V: Hash + Eq + FromReflect + TypePath + GetTypeRegistration,
        S: TypePath + BuildHasher + Default + Send + Sync
    >
);
