// FIXME: does not work

extern crate legion;

use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;
use legion::borrow::Ref;
// use std::cell::Ref;
use legion::borrow::RefMut;
// use std::cell::RefMut;
// use std::cell::RefCell;
use legion::borrow::AtomicRefCell;
// use std::rc::Rc;

pub trait Asset: 'static + Any + Send + Sync {}
impl<T> Asset for T where T: 'static + Any + Send + Sync {}

pub struct Fetch<'a, T>
where
    T: Asset,
{
    inner: Ref<'a, Box<dyn Any>>,
    _marker: PhantomData<T>,
}

impl<T> Deref for Fetch<'_, T>
where
    T: Asset,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = &**self.inner;
        inner.downcast_ref::<T>().unwrap_or_else(|| {
            panic!(
                "Unable to downcast the asset!: {}",
                std::any::type_name::<T>()
            )
        })
    }
}

pub struct FetchMut<'a, T>
where
    T: Asset,
{
    inner: RefMut<'a, Box<dyn Any>>,
    _marker: PhantomData<T>,
}

impl<A> Deref for FetchMut<'_, A>
where
    A: Asset,
{
    type Target = A;

    fn deref(&self) -> &Self::Target {
        let inner = &**self.inner;
        inner.downcast_ref::<A>().unwrap_or_else(|| {
            panic!(
                "Unable to downcast the asset!: {}",
                std::any::type_name::<A>()
            )
        })
    }
}

impl<A> DerefMut for FetchMut<'_, A>
where
    A: Asset,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        let inner = &mut (*self.inner) as &mut dyn Any;
        inner.downcast_mut::<A>().unwrap_or_else(|| {
            panic!(
                "Unable to downcast the asset!: {}",
                std::any::type_name::<A>()
            )
        })
    }
}

pub struct AssetManagement {
    pub assets: HashMap<TypeId, AtomicRefCell<Box<dyn Any>>>,
}

impl AssetManagement {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn insert<A>(&mut self, asset: A)
    where
        A: Asset,
    {
        self.assets
            .insert(TypeId::of::<A>(), AtomicRefCell::new(Box::new(asset)));
    }

    pub fn get<A>(&self) -> Option<Fetch<'_, A>>
    where
        A: Asset,
    {
        Some(Fetch {
            inner: self.assets.get(&TypeId::of::<A>())?.get(),
            _marker: Default::default(),
        })
    }

    pub fn get_mut<A: Asset>(&mut self) -> Option<FetchMut<'_, A>> {
        Some(FetchMut {
            inner: self.assets.get_mut(&TypeId::of::<A>())?.get_mut(),
            _marker: Default::default(),
        })
    }
}

impl Default for AssetManagement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug)]
    pub struct MyAsset(i32);

    #[test]
    fn asset_management_test() {
        let mut assets = AssetManagement::new();
        assets.insert(MyAsset(99));

        let my_asset = assets.get::<MyAsset>().unwrap();
        let n = &*my_asset;

        assert_eq!(n.0,  99);
    }
}
