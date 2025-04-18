use {
  bevy::{prelude::*, tasks::AsyncComputeTaskPool},
  crossbeam_channel::{Receiver, Sender, bounded},
  rfd::AsyncFileDialog,
};

#[derive(Resource, Deref)]
struct Tx<T>(Sender<T>);

#[derive(Resource, Deref)]
struct Rx<T>(Receiver<T>);

#[derive(Event)]
pub struct Open {
  pub content: Vec<u8>,
}

pub fn plugin(app: &mut App) {
  let (tx, rx) = bounded(32);

  app
    .add_event::<Open>()
    .insert_resource(Tx::<Open>(tx))
    .insert_resource(Rx::<Open>(rx))
    .add_systems(Update, receive_picks::<Open>);
}

fn receive_picks<E: Event>(rx: Res<Rx<E>>, mut events: EventWriter<E>) {
  for open in rx.try_iter() {
    events.send(open);
  }
}

// #[derive(Deref, DerefMut)]
pub struct FileDialog<'w, 's, 'a> {
  commands: &'a mut Commands<'w, 's>,
  // #[deref]
  dialog: AsyncFileDialog,
}

impl<'w, 's, 'a> FileDialog<'w, 's, 'a> {
  pub fn pick_file(self) {
    self.commands.queue(|world: &mut World| {
      let tx = world.resource::<Tx<Open>>().0.clone();

      AsyncComputeTaskPool::get()
        .spawn(async move {
          let Some(file) = self.dialog.pick_file().await else {
            return;
          };

          tx.send(Open { content: file.read().await }).unwrap();
        })
        .detach();
    });
  }
}

pub trait FileDialogExt<'w, 's> {
  #[must_use]
  fn dialog<'a>(&'a mut self) -> FileDialog<'w, 's, 'a>;
}

impl<'w, 's> FileDialogExt<'w, 's> for Commands<'w, 's> {
  fn dialog<'a>(&'a mut self) -> FileDialog<'w, 's, 'a> {
    FileDialog { commands: self, dialog: AsyncFileDialog::new() }
  }
}
