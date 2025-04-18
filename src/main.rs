mod dialog;

use bevy::{
  asset::RenderAssetUsages,
  prelude::*,
  render::render_resource::{Extent3d, TextureDimension, TextureFormat},
  tasks::AsyncComputeTaskPool,
};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, pick)
    .run();
}

mod image {
  pub const W: u32 = 256;
  pub const H: u32 = 256;
}

#[derive(Resource)]
struct Process(Handle<Image>);

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
  let mut image = Image::new_fill(
    Extent3d { width: image::W, height: image::H, depth_or_array_layers: 1 },
    TextureDimension::D2,
    &Srgba::BLACK.to_u8_array(),
    TextureFormat::Rgba8UnormSrgb,
    RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
  );

  let handle = images.add(image);

  commands.insert_resource(Process(handle.clone()));
  commands.spawn(Sprite::from_image(handle));
}

use rfd::AsyncFileDialog;

fn pick(input: Res<ButtonInput<KeyCode>>) {
  let task = async move {
    AsyncFileDialog::new().pick_file().await;
  };

  if input.just_pressed(KeyCode::Space) {
    AsyncComputeTaskPool::get().spawn(task).detach();
  }
}
