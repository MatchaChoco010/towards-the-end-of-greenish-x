use anyhow::Result;
use ggez::graphics::Image;
use ggez::*;
use std::collections::HashMap;
use std::path::Path;
use thiserror::*;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ImageStoreError {
    #[error("no such image is loaded: {0}")]
    NoSuchImage(Uuid),
    #[error("no such name image is loaded: {0}")]
    NoSuchNameImageLoaded(String),
}

pub struct ImageStore {
    image_hashmap: HashMap<Uuid, Image>,
    name_hashmap: HashMap<String, Uuid>,
}
impl ImageStore {
    pub fn new() -> Self {
        Self {
            image_hashmap: HashMap::new(),
            name_hashmap: HashMap::new(),
        }
    }

    pub fn load_image(
        &mut self,
        ctx: &mut Context,
        name: impl ToString,
        path: impl AsRef<Path>,
    ) -> Result<()> {
        let image = Image::new(ctx, path)?;
        let uuid = Uuid::new_v4();
        self.image_hashmap.insert(uuid, image);
        self.name_hashmap.insert(name.to_string(), uuid);
        Ok(())
    }

    pub fn get_image(&self, uuid: &Uuid) -> Result<&Image> {
        Ok(self
            .image_hashmap
            .get(uuid)
            .ok_or(ImageStoreError::NoSuchImage(*uuid))?)
    }

    pub fn get_image_uuid_from_name(&self, name: impl ToString) -> Result<Uuid> {
        if let Some(uuid) = self.name_hashmap.get(&name.to_string()) {
            Ok(*uuid)
        } else {
            Err(ImageStoreError::NoSuchNameImageLoaded(name.to_string()).into())
        }
    }
}
