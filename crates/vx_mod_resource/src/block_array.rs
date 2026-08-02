use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    platform::collections::HashMap,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

#[derive(Clone, Resource, Debug)]
pub struct BlockTextureArray {
    id_mapping: HashMap<String, u32>,
    texture: Handle<Image>,
}

impl BlockTextureArray {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "anything beyond u32::MAX would break the texture array"
    )]
    pub(crate) fn build(
        textures: &HashMap<String, Handle<Image>>,
        asset_server: &AssetServer,
        images: &Assets<Image>,
    ) -> Self {
        let ids: Vec<_> = textures.keys().collect();
        let layer_count = ids.len() as u32;

        let tile_size = textures
            .values()
            .next()
            .map(|img| {
                images
                    .get(img)
                    .expect("this should be run after all images are loaded")
                    .width()
            })
            .expect("should have at least on block texture");

        let mut data = Vec::with_capacity((tile_size * tile_size * 4 * layer_count) as usize);
        let mut id_mapping = HashMap::new();

        for (i, &id) in ids.iter().enumerate() {
            let image = images
                .get(&textures[id])
                .expect("this should be run after all images are loaded");

            assert_eq!(
                image.width(),
                tile_size,
                "texture `{id}` doesn't match expected tile size {tile_size}"
            );
            assert_eq!(image.height(), tile_size);

            data.extend_from_slice(
                image
                    .data
                    .as_deref()
                    .expect("block textures shouldn't be storage textures"),
            );

            id_mapping.insert(id.clone(), i as u32);
        }

        let mut image = Image::new(
            Extent3d {
                width: tile_size,
                height: tile_size * layer_count,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );

        image
            .reinterpret_stacked_2d_as_array(layer_count)
            .expect("Should be able to convert all images to a texture array");

        image.sampler = ImageSampler::nearest();

        Self {
            id_mapping,
            texture: asset_server.add(image),
        }
    }

    #[must_use]
    pub const fn texture_handle(&self) -> &Handle<Image> {
        &self.texture
    }

    #[must_use]
    pub fn get_index(&self, texture_name: &str) -> Option<u32> {
        self.id_mapping.get(texture_name).copied()
    }
}
