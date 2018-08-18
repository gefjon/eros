use crate::gfx_prelude::{Factory, Glyphs};
use std::{
    cell::RefCell,
    collections::{HashMap},
    path::{Path, PathBuf},
    rc::{Rc, Weak},
};

pub enum Resource {
    Font(Glyphs<'static>),
}

impl Resource {
    fn load(name: &Path, factory: Factory) -> Result<Self, std::io::Error> {
        match name
            .extension()
            .map(std::ffi::OsStr::to_str)
            .map(Option::unwrap)
        {
            Some("ttf") | Some("otf") => Ok(Resource::Font(Glyphs::new(
                name,
                factory,
                gfx_graphics::TextureSettings::new(),
            )?)),
            _ => unimplemented!(),
        }
    }
}

pub struct Resources {
    res: RefCell<HashMap<PathBuf, Weak<Resource>>>,
    preloads: RefCell<HashMap<PathBuf, Rc<Resource>>>,
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
    pub fn try_get(&self, resource_name: &Path) -> Option<Rc<Resource>> {
        if let Some(rc) = self.preloads.borrow_mut().remove(resource_name) {
            Some(rc)
        } else {
            self.res.borrow().get(resource_name)?.upgrade()
        }
    }
    pub fn get(&self, resource_name: &Path) -> Rc<Resource> {
        if let Some(rc) = self.preloads.borrow_mut().remove(resource_name) {
            rc
        } else if let Some(Some(rc)) = self.res.borrow().get(resource_name).map(Weak::upgrade) {
            rc
        } else {
            let rc = Rc::new(Resource::load(resource_name, self.factory.clone()).unwrap());
            self.res
                .borrow_mut()
                .insert(resource_name.to_owned(), Rc::downgrade(&rc));
            rc
        }
    }
    pub fn preload(&self, resource_name: &Path) {
        if self.preloads.borrow().contains_key(resource_name) {
            ()
        } else {
            let rc = Rc::new(Resource::load(resource_name, self.factory.clone()).unwrap());
            self.res
                .borrow_mut()
                .insert(resource_name.to_owned(), Rc::downgrade(&rc));
            self.preloads
                .borrow_mut()
                .insert(resource_name.to_owned(), rc);
        }
    }
}
