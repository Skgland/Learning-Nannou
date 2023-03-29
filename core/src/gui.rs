use derive_macros_helpers::{Bounded, Enumerable};
use log::error;
use nannou::prelude::*;
use std::fmt::Debug;

pub trait Application<'a> {
    type ViewResult;
    type RawEventResult;
    type UpdateResult;

    fn view(
        &self,
        app: &App,
        frame: &Frame,
    ) -> Self::ViewResult {
        todo!()
    }


    fn raw_window_event(&mut self, app: &App, event: &nannou::winit::event::WindowEvent) -> Self::RawEventResult{
        todo!()
    }

    fn update(
        &mut self,
        app: &App,
        update: Update,
        main_window: WindowId,
    ) -> Self::UpdateResult {
        todo!()
    }
}

pub type TextureMap<K> = std::collections::btree_map::BTreeMap<K, wgpu::Texture>;

pub fn load_textures<K: Ord + Debug + Enumerable + Bounded + ToString>(app: &App) -> TextureMap<K>
{
    let mut texture_map = <TextureMap<_>>::new();

    let texture_assets = app.assets_path().unwrap().join("textures");

    for tile_index in K::enumerate_all() {
        let file_name = tile_index.to_string();
        let path = texture_assets.join(format!("{file_name}.png"));
        if let Ok(texture) = wgpu::Texture::from_path(app, &path){
            texture_map.insert(tile_index, texture);
        } else {
            error!(
                "Failed loading Texture with Index: {:?} , at: {:?}",
                &tile_index, path.display()
            );
        }
    }

    texture_map
}
