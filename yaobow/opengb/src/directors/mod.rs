mod adv_director;
mod global_state;
mod persistent_state;
mod sce_commands;
mod sce_vm;

use self::sce_vm::SceState;
use crate::{
    comdef::{IRoleController, IScnSceneComponent},
    scene::RoleController,
};
pub use adv_director::AdventureDirector;
use crosscom::ComRc;
pub use global_state::GlobalState;
pub use persistent_state::PersistentState;
use radiance::comdef::{IEntity, ISceneManager};
pub use sce_vm::{SceExecutionOptions, SceProcHooks};

pub trait SceneManagerExtensions {
    fn scn_scene(&self) -> Option<ComRc<IScnSceneComponent>>;

    fn get_resolved_role(&self, state: &SceState, role_id: i32) -> Option<ComRc<IEntity>> {
        let resolved_role_id = if role_id == -1 {
            state.global_state().role_controlled()
        } else {
            role_id
        };
        self.scn_scene()
            .unwrap()
            .get()
            .get_role_entity(resolve_role_id(state, role_id))
    }

    fn resolve_role_do<T, F: Fn(ComRc<IEntity>, ComRc<IRoleController>) -> T>(
        &self,
        state: &SceState,
        role_id: i32,
        action: F,
    ) -> Option<T> {
        let role = self.get_resolved_role(state, role_id);
        if let Some(r) = role {
            let role_model = RoleController::get_role_controller(r.clone()).unwrap();
            Some(action(r, role_model))
        } else {
            log::error!("Cannot find role {}", role_id);
            None
        }
    }

    fn resolve_role_mut_do<T, F: Fn(ComRc<IEntity>, ComRc<IRoleController>) -> T>(
        &self,
        state: &SceState,
        role_id: i32,
        action: F,
    ) -> Option<T> {
        let role = self.get_resolved_role(state, role_id);
        if let Some(r) = role {
            let role_model = RoleController::get_role_controller(r.clone()).unwrap();
            Some(action(r, role_model))
        } else {
            log::error!("Cannot find role {}", role_id);
            None
        }
    }
}

impl SceneManagerExtensions for ComRc<ISceneManager> {
    fn scn_scene(&self) -> Option<ComRc<IScnSceneComponent>> {
        self.scene()?
            .get_component(IScnSceneComponent::uuid())?
            .query_interface::<IScnSceneComponent>()
    }
}

fn resolve_role_id(state: &SceState, role_id: i32) -> i32 {
    if role_id == -1 {
        state.global_state().role_controlled()
    } else {
        role_id
    }
}
