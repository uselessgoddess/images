mod dialog;

use bevy::{
  asset::RenderAssetUsages,
  prelude::*,
  render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(dialog::plugin)
    .add_systems(Startup, setup)
    .add_systems(Update, (pick, update))
    .run();
}

mod dim {
  pub const W: u32 = 256;
  pub const H: u32 = 256;
}

fn render_usage() -> RenderAssetUsages {
  RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD
}

#[derive(Resource, Deref)]
struct Process(Handle<Image>);

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
  commands.spawn(Camera2d);

  let image = Image::new_fill(
    Extent3d { width: dim::W, height: dim::H, depth_or_array_layers: 1 },
    TextureDimension::D2,
    &Srgba::WHITE.to_u8_array(),
    TextureFormat::Rgba8UnormSrgb,
    render_usage(),
  );

  let handle = images.add(image);

  commands.insert_resource(Process(handle.clone()));
  commands.spawn(Sprite::from_image(handle));
}

use {
  dialog::{FileDialogExt, Open},
  image::imageops::FilterType,
};

fn pick(input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
  if input.just_pressed(KeyCode::Space) {
    commands.dialog().pick_file();
  }
}

fn update(
  mut events: EventReader<Open>,
  mut images: ResMut<Assets<Image>>,
  process: Res<Process>,
) {
  for Open { content } in events.read() {
    if let Ok(dynamic) = image::load_from_memory(&content) {
      let Some(image) = images.get_mut(process.id()) else { continue };

      let dynamic = dynamic.resize_exact(dim::W, dim::H, FilterType::Nearest);

      *image = Image::from_dynamic(dynamic, true, render_usage());
    }
  }
}
