use bevy_app::Plugin;
use bevy_ecs::{
    prelude::{Changed, Query},
    schedule::ParallelSystemDescriptorCoercion,
};
use bevy_transform::{prelude::GlobalTransform, TransformSystem};
use superconductor::components::Instance;

pub struct SuperconductorTransformPlugin;

impl Plugin for SuperconductorTransformPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_system(sync_transform.after(TransformSystem::TransformPropagate));
    }
}

// fn sync_camera(
//     bevy_camera: Res<ActiveCamera<Camera3d>>,
//     gtf_q: Query<&GlobalTransform>,
//     mut spr_camera: ResMut<superconductor::resources::Camera>,
// ) {
//     let bcam_e = bevy_camera.get().expect("has ActiveCamera for 3d");
//     let gtf = gtf_q.get(bcam_e).expect("bcam e has gtf");
//     spr_camera.position =
// }

fn sync_transform(
    mut gtf_w_instance_q: Query<(&GlobalTransform, &mut Instance), Changed<GlobalTransform>>,
) {
    for (gtf, mut ins) in gtf_w_instance_q.iter_mut() {
        let instance = &mut ins.0;
        instance.position = gtf.translation;
        instance.rotation = gtf.rotation;
        if !(gtf.scale.x == gtf.scale.y && gtf.scale.y == gtf.scale.z) {
            panic!("superconductor uses uniform scale");
        }
        instance.scale = gtf.scale.x;
    }
}
