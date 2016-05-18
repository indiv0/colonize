use std::cell::RefCell;
use std::collections::BTreeMap;
use std::convert::AsRef;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use glium::backend::Facade;

pub type Map<T> = BTreeMap<PathBuf, Rc<T>>;

pub trait Resource {
    fn load<F>(&F, &Path) -> Self
        where F: Facade;
}

pub struct Manager<F, T>
    where F: Facade,
          T: Resource,
{
    pub map: RefCell<Map<T>>,
    asset_path: PathBuf,
    facade: F,
}

impl<F, T> Manager<F, T>
    where F: Facade,
          T: Resource,
{
    pub fn new(facade: F, asset_path: PathBuf) -> Manager<F, T> {
        Manager {
            map: RefCell::new(Map::new()),
            asset_path: asset_path,
            facade: facade,
        }
    }

    pub fn load<P>(&self, key: P) -> Rc<T>
        where P: AsRef<Path>,
    {
        let key = self.asset_path.join(key);

        let mut map = self.map.borrow_mut();

        // Look up if the resource with the given key is already loaded. If it
        // is, return it.
        if let Some(value) = map.get(&key) {
            return value.clone()
        }

        // Load the resource with the requested filename and save it into the
        // map.
        let resource = Rc::new(T::load(&self.facade, &key));
        map.insert(key, resource.clone());

        // Return a copy of the loaded resource.
        resource
    }
}
