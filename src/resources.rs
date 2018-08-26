use crate::gfx_prelude::{Factory, Glyphs};
use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
    rc::{Rc, Weak},
};

#[derive(Clone)]
pub enum Resource {
    Font(Rc<RefCell<Glyphs<'static>>>),
    _DummyResource,
}

#[derive(Clone)]
pub enum WeakResource {
    Font(Weak<RefCell<Glyphs<'static>>>),
    _DummyResource,
}

impl Resource {
    fn load(name: &Path, factory: Factory) -> Result<Self, std::io::Error> {
        match name
            .extension()
            .map(std::ffi::OsStr::to_str)
            .map(Option::unwrap)
        {
            Some("ttf") | Some("otf") => {
                let f = Glyphs::new(name, factory, gfx_graphics::TextureSettings::new())?;
                Ok(Resource::Font(Rc::new(RefCell::new(f))))
            }
            _ => unimplemented!(),
        }
    }
    pub fn downgrade(res: &Resource) -> WeakResource {
        match res {
            Resource::Font(ref rc) => WeakResource::Font(Rc::downgrade(rc)),
            Resource::_DummyResource => WeakResource::_DummyResource,
        }
    }
    pub fn try_as_glyphs(&self) -> Option<Rc<RefCell<Glyphs<'static>>>> {
        if let Resource::Font(ref rc) = self {
            Some(rc.clone())
        } else {
            None
        }
    }
}

impl WeakResource {
    pub fn upgrade(&self) -> Option<Resource> {
        match self {
            WeakResource::Font(ref weak) => Some(Resource::Font(weak.upgrade()?)),
            WeakResource::_DummyResource => Some(Resource::_DummyResource),
        }
    }
}

pub struct Resources {
    res: RefCell<HashMap<PathBuf, WeakResource>>,
    preloads: RefCell<HashMap<PathBuf, Resource>>,
    factory: Factory,
}

impl Resources {
    pub fn new(factory: Factory) -> Self {
        Self {
            res: RefCell::new(HashMap::new()),
            preloads: RefCell::new(HashMap::new()),
            factory,
        }
    }
    pub fn from_capacity(cap: usize, factory: Factory) -> Self {
        Self {
            res: RefCell::new(HashMap::with_capacity(cap)),
            preloads: RefCell::new(HashMap::new()),
            factory,
        }
    }
    pub fn from_preload_capacity(preload_cap: usize, factory: Factory) -> Self {
        Self {
            res: RefCell::new(HashMap::new()),
            preloads: RefCell::new(HashMap::with_capacity(preload_cap)),
            factory,
        }
    }
    pub fn from_capcity_and_preload_capacity(
        cap: usize,
        preload_cap: usize,
        factory: Factory,
    ) -> Self {
        Self {
            res: RefCell::new(HashMap::with_capacity(cap)),
            preloads: RefCell::new(HashMap::with_capacity(preload_cap)),
            factory,
        }
    }
    pub fn try_get(&self, resource_name: &Path) -> Option<Resource> {
        if let Some(res) = self.preloads.borrow_mut().remove(resource_name) {
            Some(res)
        } else {
            self.res.borrow().get(resource_name)?.upgrade()
        }
    }
    pub fn get(&self, resource_name: &Path) -> Resource {
        if let Some(res) = self.preloads.borrow_mut().remove(resource_name) {
            return res;
        }

        {
            if let Some(Some(res)) = self
                .res
                .borrow()
                .get(resource_name)
                .map(WeakResource::upgrade)
            {
                return res;
            }
        }

        if let Some(Some(res)) = self
            .res
            .borrow()
            .get(resource_name)
            .map(WeakResource::upgrade)
        {
            return res;
        }

        let loaded = Resource::load(resource_name, self.factory.clone()).unwrap();
        self.res
            .borrow_mut()
            .insert(resource_name.to_owned(), Resource::downgrade(&loaded));
        loaded
    }
    pub fn preload(&self, resource_name: &Path) {
        if self.preloads.borrow().contains_key(resource_name) {
            ()
        } else {
            let loaded = Resource::load(resource_name, self.factory.clone()).unwrap();
            self.res
                .borrow_mut()
                .insert(resource_name.to_owned(), Resource::downgrade(&loaded));
            self.preloads
                .borrow_mut()
                .insert(resource_name.to_owned(), loaded);
        }
    }
}
