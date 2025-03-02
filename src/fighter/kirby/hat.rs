use crate::imports::imports_agent::*;

pub unsafe extern "C" fn sonic_hat(weapon: &mut L2CWeaponCommon, status_kind: i32, ball_mode: bool) {
    let invis = status_kind == *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_HOMING_START || ball_mode;
    if (*FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N..*FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_CANCEL).contains(&status_kind) 
    && invis {
        let scale = smash::phx::Vector3f { x: 0.00001, y: 0.00001, z: 0.00001 };
        ModelModule::set_joint_scale(weapon.module_accessor, Hash40::new("rot"), &scale);
    }
}
pub unsafe extern "C" fn hat_update(weapon: &mut L2CWeaponCommon) {
    let owner = get_owner_boma(weapon);
    let copy_kind = WorkModule::get_int(&mut *owner, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
    let status_kind = StatusModule::status_kind(&mut *owner);
    if copy_kind == *FIGHTER_KIND_SONIC {
        let ball_mode = WorkModule::is_flag(owner, sonic::SPECIAL_TORNADO_BALL);
        sonic_hat(weapon,status_kind,ball_mode);
    }
}

pub fn install() {
    Agent::new("kirby_hat")
        .on_line(Main,  hat_update)
    .install();
}
