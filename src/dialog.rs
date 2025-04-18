use bevy::prelude::*;

use crossbeam_channel::{Receiver, Sender, bounded};

#[derive(Resource)]
struct Tx<T>(Sender<T>);

#[derive(Resource)]
struct Rx<T>(Receiver<T>);

pub fn plugin(app: &mut App) {}
