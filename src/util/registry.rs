use std::collections::HashMap;
use std::any::TypeId;

/// A type registry assigning concrete types to a numeric(`usize`),
/// unique identifier. Also stores meta data `T` about the registered
/// type.
#[derive(Debug, Clone)]
pub struct Registry<T = ()>
{
    /// Maps key `TypeId` to its ID, using a `HashMap` as `TypeId`s
    /// are non-contiguous.
    map: HashMap<TypeId, usize>,
    /// Maps `usize` ID to key `TypeId`, which can be cheaply done
    /// using a `Vec` as opposed to a `HashMap`.
    rev: Vec<(TypeId, T)>,
}

impl<T> Registry<T>
{
    /// Registers the given type and its meta data, if not already present
    /// in the registry.
    pub fn register<K: 'static>(&mut self, meta: T)
    {
        let type_id = TypeId::of::<K>();

        // Don't register duplicate types
        if !self.map.contains_key(&type_id)
        {
            // Assigned ID's are incremental, starting at 0
            self.map.insert(type_id, self.rev.len());
            self.rev.push((type_id, meta));
        }
    }

    /// Get the ID of the given type, if present in this map.
    pub fn id<K: 'static>(&self) -> Option<usize>
    {
        self.map
            .get(&TypeId::of::<K>())
            .copied()
    }

    /// Get the type and meta data entry of the given identifier, if present
    /// in this map.
    pub fn get(&self, id: usize) -> Option<&(TypeId, T)>
    {
        self.rev.get(id)
    }

    /// [Registry::get] without bounds checking
    pub unsafe fn get_unchecked(&self, id: usize) -> &(TypeId, T)
    {
        self.rev.get_unchecked(id)
    }
}

impl<T> Default for Registry<T>
{
    fn default() -> Self
    {
        Self
        {
            map: Default::default(),
            rev: Default::default(),
        }
    }
}