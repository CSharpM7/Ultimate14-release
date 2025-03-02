// status imports
use crate::imports::imports_agent::*;

unsafe extern "C" fn escape_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if GroundModule::is_passable_ground(fighter.module_accessor) {
        let pass_flick_y = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("pass_flick_y"));
        let frame = MotionModule::frame(fighter.module_accessor);
        let pass_frame = 3.0;
        let max_frame = 15.0;

        if ControlModule::get_stick_y(fighter.module_accessor) <= -0.5 
        && ControlModule::get_flick_y(fighter.module_accessor) < pass_flick_y 
        && pass_frame <= frame && frame <= max_frame {
            GroundModule::pass_floor(fighter.module_accessor);
            GroundModule::attach_ground(fighter.module_accessor, false);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
            
            ControlModule::clear_command(fighter.module_accessor, true);
            return 1.into();
        };
    }
    0.into()
}

//#[skyline::hook(replace = L2CFighterCommon_status_EscapeAir)]
unsafe extern "C" fn status_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_escape_air_common();

    if StatusModule::prev_status_kind(fighter.module_accessor,0) == *FIGHTER_STATUS_KIND_ESCAPE {
        let frame = MotionModule::frame(fighter.module_accessor);
        let rate = MotionModule::rate(fighter.module_accessor);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), frame+7.0,rate, false, 0.0, false, false);
        //MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 3.0, 1.0, false, 0.0, false, false);
        GroundModule::pass_floor(fighter.module_accessor);
        //GroundModule::attach_ground(fighter.module_accessor, false);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        GroundModule::set_passable_check(fighter.module_accessor, true);

        let control_x = ControlModule::get_stick_x(fighter.module_accessor);
        let speed_x = control_x*0.75;
        SET_SPEED_EX(fighter,speed_x,-2.0,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);

        let y_offset = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0) * 0.5;
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, y_offset, 2, -90, 0, 0, 1.0, true);
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air_slide"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 0.0, 1.0, false, 0.0, false, false);
    }

    fighter.sub_shift_status_main(L2CValue::Ptr(escape_air_main_loop as *const () as _))
}

unsafe extern "C" fn escape_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::prev_status_kind(fighter.module_accessor,0) == *FIGHTER_STATUS_KIND_ESCAPE {
        let frame = MotionModule::frame(fighter.module_accessor);
        if (frame < 3.0)
        {
            macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        }
        //GroundModule::pass_floor(fighter.module_accessor);
        GroundModule::set_passable_check(fighter.module_accessor, true);
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_y = (-1.0 as f32).min(KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
        SET_SPEED_EX(fighter,speed_x,speed_y,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::resume_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    if !fighter.sub_escape_air_common_main().get_bool() {
        fighter.sub_escape_check_rumble();
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_ESCAPE, escape_exec);
    agent.status(Main, *FIGHTER_STATUS_KIND_ESCAPE_AIR, status_EscapeAir);
}
