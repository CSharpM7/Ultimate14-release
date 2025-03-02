mod acmd;
mod status;
mod agent_init;
//mod opff;

pub fn install() {
    #[cfg(not(feature = "devhook"))]
    dev_install();
    #[cfg(not(feature = "dev"))]
    hook_install();
}
pub fn hook_install() {
}
pub fn dev_install() {
    let agent = &mut smashline::Agent::new("sonic");
    acmd::install(agent);
    status::install(agent);
    agent_init::install(agent);
    //opff::install(agent);
    agent.install();
}

pub unsafe fn set_ball_mode(agent: &mut smashline::L2CAgentBase, active: bool) {
    use crate::imports::imports_acmd::*;
    println!("Ball mode: {active}");
    WorkModule::set_flag(agent.module_accessor,active,sonic::SPECIAL_TORNADO_BALL);
    ItemModule::set_attach_item_visibility(agent.module_accessor, !active, 0);
    let is_kirby = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if is_kirby {
        if !active
        || StatusModule::status_kind(agent.module_accessor) == *FIGHTER_KIRBY_STATUS_KIND_SONIC_SPECIAL_N_HOMING_START {
            ColorBlendModule::cancel_main_color(agent.module_accessor, 0);
        }
        else {
            let color1 = Vector4f{ /* Red */ x: 0.06, /* Green */ y: 0.125, /* Blue */ z: 1.0, /* Alpha */ w: 1.0};
            let color2 = Vector4f{ /* Red */ x: 0.06, /* Green */ y: 0.125, /* Blue */ z: 1.0, /* Alpha */ w: 1.0};
            ColorBlendModule::set_main_color(agent.module_accessor, /* Brightness */ &color1, /* Diffuse */ &color2, 0.51, 0.9, 0, /* Display Color */ true);
        }
        return;
    }

    let vismode = if active {hash40("body_sphere")} else {hash40("body_normal")};
    VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, vismode as i64);
    let body_status = if active {*HIT_STATUS_OFF} else {*HIT_STATUS_NORMAL};
    let rot_status = if !active {*HIT_STATUS_OFF} else {*HIT_STATUS_NORMAL};
    macros::HIT_NODE(agent, Hash40::new("waist"), body_status);
    macros::HIT_NODE(agent, Hash40::new("head"), body_status);
    macros::HIT_NODE(agent, Hash40::new("s_stingd1"), body_status);
    macros::HIT_NODE(agent, Hash40::new("shoulderr"), body_status);
    macros::HIT_NODE(agent, Hash40::new("shoulderl"), body_status);
    macros::HIT_NODE(agent, Hash40::new("armr"), body_status);
    macros::HIT_NODE(agent, Hash40::new("arml"), body_status);
    macros::HIT_NODE(agent, Hash40::new("legr"), body_status);
    macros::HIT_NODE(agent, Hash40::new("legl"), body_status);
    macros::HIT_NODE(agent, Hash40::new("kneer"), body_status);
    macros::HIT_NODE(agent, Hash40::new("kneel"), body_status);
    macros::HIT_NODE(agent, Hash40::new("footr"), body_status);
    macros::HIT_NODE(agent, Hash40::new("footl"), body_status);
    macros::HIT_NODE(agent, Hash40::new("rot"), rot_status);
}