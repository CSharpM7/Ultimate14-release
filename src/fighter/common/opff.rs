use crate::imports::imports_agent::*;

pub unsafe extern "C" fn held_clatter_escape(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if ControlModule::get_clatter_time(boma, 0) > 0.0 {
        if (&[
            *FIGHTER_STATUS_KIND_BURY_WAIT,
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
            
            *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI,
            *FIGHTER_STATUS_KIND_CATCHED_RIDLEY
        ]).contains(&status_kind)
        || true
        {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)
            || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                ControlModule::add_clatter_time(boma, -2.0, 0);
            }

            //println!("Clatter: {}",ControlModule::get_clatter_time(boma, 0));
        }
    }
}
pub unsafe extern "C" fn dash_drop(boma: &mut BattleObjectModuleAccessor, status_kind: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status_kind) {
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind) && frame < 3.0 {
            return;
        } 
        if GroundModule::is_passable_ground(boma) {
            let pass_flick_y = WorkModule::get_param_int(boma, hash40("common"), hash40("pass_flick_y"));

            if ControlModule::get_stick_y(boma) <= -0.5 && ControlModule::get_flick_y(boma) < pass_flick_y {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_PASS, true);
            };
        }
    }
}
pub unsafe extern "C" fn disable_jump(boma: &mut BattleObjectModuleAccessor, battle_object: &mut BattleObject) { 
    if VarModule::is_flag(battle_object, fighter::instance::flag::DISABLE_JUMP) {
        WorkModule::unable_transition_term_group_ex(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::unable_transition_term_group_ex(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        WorkModule::unable_transition_term_group_ex(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    }
}
pub unsafe extern "C" fn track_momentum(boma: &mut BattleObjectModuleAccessor, battle_object: &mut BattleObject) {
    if StatusModule::situation_kind(boma) != *SITUATION_KIND_GROUND {
        return;
    }
    let mut speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if [*FIGHTER_STATUS_KIND_DASH,*FIGHTER_STATUS_KIND_TURN_DASH].contains(&StatusModule::prev_status_kind(boma, 0)) {
        let frame = MotionModule::frame(boma);
        let threshold = 10.0;
        let ratio = if frame < threshold {frame/threshold} else {1.0};
        let stable_speed = WorkModule::get_param_float(boma, smash::hash40("air_speed_x_stable"), 0);
        speed_x = lerp(stable_speed,speed_x.abs(),ratio)*speed_x.signum();
    }
    VarModule::set_float(battle_object,fighter::instance::float::CURRENT_SPEED_X,speed_x);
}

unsafe extern "C" fn fighter_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let battle_object = smash::app::sv_system::battle_object(fighter.lua_state_agent);
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = smash::app::utility::get_kind(boma);
    let frame = MotionModule::frame(boma);

    held_clatter_escape(boma,status_kind);
    //dash_drop(boma,status_kind,frame);
    disable_jump(boma,battle_object);
    track_momentum(boma,battle_object);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main,  fighter_frame);
}