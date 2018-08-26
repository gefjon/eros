use crate::gfx_prelude::{Factory, Glyphs};
use log::*;
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
    keep: RefCell<HashMap<PathBuf, Resource>>,
    factory: Factory,
}

pub struct ResourcesBuilder {
    preload_cap: Option<usize>,
    res_cap: Option<usize>,
    keep_cap: Option<usize>,
    factory: Factory,
}

impl ResourcesBuilder {
    pub fn new(factory: Factory) -> Self {
        Self {
            preload_cap: None,
            res_cap: None,
            keep_cap: None,
            factory,
        }
    }
    pub fn build(self) -> Resources {
        let r = Resources::new(self.factory);
        if let Some(preload_cap) = self.preload_cap {
            r.preloads.borrow_mut().reserve(preload_cap);
        }
        if let Some(res_cap) = self.res_cap {
            r.res.borrow_mut().reserve(res_cap);
        }
        if let Some(keep_cap) = self.keep_cap {
            r.keep.borrow_mut().reserve(keep_cap);
        }
        r
    }
}

impl Resources {
    fn new(factory: Factory) -> Self {
        Self {
            res: RefCell::new(HashMap::new()),
            preloads: RefCell::new(HashMap::new()),
            keep: RefCell::new(HashMap::new()),
            factory,
        }
    }
    pub fn try_get(&self, resource_name: &Path) -> Option<Resource> {
        if let Some(res) = self.preloads.borrow_mut().remove(resource_name) {
            Some(res)
        } else if let Some(res) = self.keep.borrow().get(resource_name) {
            Some(res.clone())
        } else {
            self.res.borrow().get(resource_name)?.upgrade()
        }
    }
    pub fn keep(&self, resource_name: &Path) {
        if self.keep.borrow().contains_key(resource_name) {
            info!("{:#?} is already in `keep`", resource_name);
            return;
        }

        if let Some(res) = self.preloads.borrow_mut().remove(resource_name) {
            info!("{:#?} is in `preloads`", resource_name);
            self.keep.borrow_mut().insert(resource_name.to_owned(), res);
        } else if let Some(Some(res)) = self
            .res
            .borrow_mut()
            .remove(resource_name)
            .as_ref()
            .map(WeakResource::upgrade)
        {
            info!("{:#?} is in `res`", resource_name);
            self.keep.borrow_mut().insert(resource_name.to_owned(), res);
        } else {
            info!("About to load & keep {:#?}", resource_name);
            self.keep.borrow_mut().insert(
                resource_name.to_owned(),
                Resource::load(resource_name, self.factory.clone()).unwrap(),
            );
            info!("Finished loading & keeping {:#?}", resource_name);
        }
    }
    pub fn get(&self, resource_name: &Path) -> Resource {
        if let Some(res) = self.preloads.borrow_mut().remove(resource_name) {
            info!("{:#?} is preloaded", resource_name);
            return res;
        }

        if let Some(res) = self.keep.borrow().get(resource_name) {
            info!("{:#?} is kept", resource_name);
            return res.clone();
        }

        {
            if let Some(Some(res)) = self
                .res
                .borrow()
                .get(resource_name)
                .map(WeakResource::upgrade)
            {
                info!("{:#?} is cached", resource_name);
                return res;
            }
        }

        if let Some(Some(res)) = self
            .res
            .borrow()
            .get(resource_name)
            .map(WeakResource::upgrade)
        {
            info!("{:#?} is not cached, loading it", resource_name);
            return res;
        }

        let loaded = Resource::load(resource_name, self.factory.clone()).unwrap();
        self.res
            .borrow_mut()
            .insert(resource_name.to_owned(), Resource::downgrade(&loaded));
        info!("Finished loading {:#?}", resource_name);
        loaded
    }
    pub fn preload(&self, resource_name: &Path) {
        if self.preloads.borrow().contains_key(resource_name) {
            info!("{:#?} is already preloaded", resource_name);
            ()
        } else {
            info!("about to preload {:#?}", resource_name);
            let loaded = Resource::load(resource_name, self.factory.clone()).unwrap();
            self.res
                .borrow_mut()
                .insert(resource_name.to_owned(), Resource::downgrade(&loaded));
            self.preloads
                .borrow_mut()
                .insert(resource_name.to_owned(), loaded);
            info!("finished preloading {:#?}", resource_name);
        }
    }
}
