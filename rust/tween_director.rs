// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2026 Khvor' Gates contributors
// Portions derived from Phantom Camera 0.11.0.3 (MIT, Marcus Skov) addons/phantom_camera/scripts/phantom_camera/phantom_camera_tween_director.gd

//! A place in a scene to declare how the blend between particular virtual cameras should look.
//!
//! Intent: a designer wants the move from one camera to another to differ from that camera's
//! usual blend, such as a slow pan into a cutscene or an instant cut out of a menu, and sets
//! it here as data for those cameras instead of in code.
//!
//! Responsibility: it holds those rules and makes them known to the addon's shared registry
//! while it is in the tree. Applying a rule when a blend starts is the host's job.
//!
//! Why a node: the rules belong to a scene and should apply exactly while that scene is loaded,
//! which is what a place in the tree gives.
//!
//! Why a new type: a built-in node cannot carry the rules, and a resource on its own has no
//! presence in the tree to switch them on and off with the scene.

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
