// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 Khvor' Gates contributors
// Portions derived from Phantom Camera 0.11.0.3 (MIT, Marcus Skov) addons/phantom_camera/scripts/phantom_camera/phantom_camera_tween_director.gd
//

//! A place to say how particular changes of shot should look, instead of letting
//! every transition use the target camera's own setting.
//!
//! Intent: a designer who wants a slow pan from the overview into the action, but a
//! hard cut back out, adds one of these to the scene and lists those exceptions. The
//! cameras stay untouched.
//!
//! Responsibility: it holds the designer's list of transition overrides and makes
//! them known to the addon while it is in the tree. Picking and running a transition
//! stays with the bridge under the real camera.
//!
//! Why a node: the overrides belong to a scene, and they should apply exactly while
//! that scene is loaded, which the tree already tracks.
//!
//! Why a new type: a resource alone has no presence in the tree to come and go with
//! a scene, and putting the overrides on the cameras would split one rule about a
//! pair of shots across both of them.

use godot::classes::{INode, Node};
use godot::prelude::*;

use crate::manager::PhantomCameraManager;
use crate::resources::TweenDirectorResource;

#[derive(GodotClass)]
#[class(base=Node, tool)]
pub struct PhantomCameraTweenDirector {
    base: Base<Node>,

    #[export]
    #[var(get = get_tween_director, set = set_tween_director)]
    tween_director: Array<Gd<TweenDirectorResource>>,
}

#[godot_api]
impl INode for PhantomCameraTweenDirector {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            tween_director: Array::new(),
        }
    }

    fn enter_tree(&mut self) {
        PhantomCameraManager::notify_tween_director_added(&self.to_gd().upcast::<Node>());
    }

    fn exit_tree(&mut self) {
        PhantomCameraManager::notify_tween_director_removed(&self.to_gd().upcast::<Node>());
    }
}

#[godot_api]
impl PhantomCameraTweenDirector {
    #[func]
    pub fn set_tween_director(&mut self, value: Array<Gd<TweenDirectorResource>>) {
        self.tween_director = value;
    }

    #[func]
    pub fn get_tween_director(&self) -> Array<Gd<TweenDirectorResource>> {
        self.tween_director.clone()
    }
}

impl PhantomCameraTweenDirector {
    pub fn director_resources(&self) -> Array<Gd<TweenDirectorResource>> {
        self.tween_director.clone()
    }
}
