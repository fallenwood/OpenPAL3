use crate::directors::sce_vm::{SceCommand, SceState};
use crosscom::ComRc;
use imgui::Ui;
use radiance::comdef::ISceneManager;

#[derive(Debug, Clone)]
pub struct SceCommandGetCombat {
    var: i16,
}

impl SceCommand for SceCommandGetCombat {
    fn update(
        &mut self,
        scene_manager: ComRc<ISceneManager>,
        ui: &Ui,
        state: &mut SceState,
        delta_sec: f32,
    ) -> bool {
        state
            .context_mut()
            .current_proc_context_mut()
            .set_local(self.var, 1);
        true
    }
}

impl SceCommandGetCombat {
    pub fn new(var: i16) -> Self {
        Self { var }
    }
}
