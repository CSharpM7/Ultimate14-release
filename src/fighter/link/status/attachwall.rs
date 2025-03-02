use crate::imports::imports_agent::*;

pub unsafe extern "C" fn attachwall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_main_stick(fighter.module_accessor);
    ControlModule::clear_command(fighter.module_accessor, true);

    let mut start_stamina = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("attach_wall_frame"));
    let cliff_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT);
    start_stamina -= (cliff_count*20);
    WorkModule::set_int(fighter.module_accessor, start_stamina as i32,*FIGHTER_INSTANCE_WORK_ID_INT_NO_ATTACH_WALL_FRAME);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("attach_wall"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        attachwall_substatus(fighter);
    }
    GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES));
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(attachwall_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(attachwall_main_loop as *const () as _))
}

unsafe extern "C" fn attachwall_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {

    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        println!("Wall Jump?");
        WorkModule::sub_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ATTACH_WALL_WORK_INT_FRAME); //30
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), false.into());
    }

    let wall_jump_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("wall_jump_stick_x"));
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    if stick_x.abs() >= wall_jump_stick_x 
    && stick_x.signum() == lr {
        println!("StickX: {stick_x} LR: {lr} WJ: {wall_jump_stick_x}");
        let next_status = if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {FIGHTER_STATUS_KIND_FALL} else {FIGHTER_STATUS_KIND_FALL_AERIAL};
        fighter.change_status(next_status.into(), false.into());
        return 0.into()
    }

    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if stick_y.abs() <= 0.25 {
        if MotionModule::motion_kind(fighter.module_accessor) != smash::hash40("attach_wall") {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("attach_wall"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
    }
    else {
        WorkModule::sub_int(fighter.module_accessor, 2, *FIGHTER_STATUS_ATTACH_WALL_WORK_INT_FRAME);
        let dir = stick_y.signum();
        if MotionModule::motion_kind(fighter.module_accessor) != smash::hash40("attach_wall_climb")
        {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("attach_wall_climb"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
        else if dir < 0.0 && MotionModule::frame(fighter.module_accessor) <= 0.0 {
            MotionModule::set_frame(fighter.module_accessor, MotionModule::end_frame(fighter.module_accessor), false);
        }
        MotionModule::set_rate(fighter.module_accessor, dir*0.2);
        //SET_SPEED_EX(fighter,0.0,dir*2.0,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        PostureModule::add_pos(fighter.module_accessor, &Vector3f{ x: 0.0, y: 0.25*dir, z:0.0});
    }
    0.into()
}

unsafe extern "C" fn attachwall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let attach_side = if 0.0 <= lr {*GROUND_TOUCH_FLAG_LEFT} else { *GROUND_TOUCH_FLAG_RIGHT };
    let max_cliff = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_max_count"));
    let enter_cliff = GroundModule::can_entry_cliff(fighter.module_accessor) != 0;
    let maxed_out = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT) >= max_cliff;
    let remove_attach = !GroundModule::is_attachable(fighter.module_accessor, GroundTouchFlag(attach_side)) || (maxed_out && enter_cliff);
    if remove_attach {
        WorkModule::set_int(fighter.module_accessor,0,*FIGHTER_INSTANCE_WORK_ID_INT_NO_ATTACH_WALL_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_FALL.into(), true.into());
        return 1.into();
    }
    else if enter_cliff {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CLIFF_CATCH_MOVE);
        fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE.into(), true.into());
        return 1.into();
    }

    let stamina = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_ATTACH_WALL_FRAME)-1;
    WorkModule::sub_int(fighter.module_accessor, 1, *FIGHTER_STATUS_ATTACH_WALL_WORK_INT_FRAME);

    if stamina <= 0 {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_FALL.into(), false.into());
        return 1.into();
    }
    else if stamina < 90 {
        if stamina == 45 {
			EFFECT_FOLLOW(fighter, Hash40::new("sys_steam3"), Hash40::new("top"), 0, 7.0, 3.0, 0, 0, 0, 1.0, true);
        }
		let sweatRate = 10.0;
		let sweatSize = 0.35;
		let modulo = stamina as f32 % sweatRate;
		if (modulo<1.0)
		{
			EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_sweat"), Hash40::new("top"), 0, 14.5, 3.0, 0, 0, 0, sweatSize, true);
		}
    }
    
    0.into()
}

unsafe extern "C" fn cliff_catch_move_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CliffCatchMove();
    let prev_was_wall = StatusModule::prev_status_kind(fighter.module_accessor,0) == *FIGHTER_STATUS_KIND_ATTACH_WALL;
    if prev_was_wall {
        let end_frame = MotionModule::end_frame(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor,2.0);
        MotionModule::set_frame(fighter.module_accessor,end_frame/2.0,false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(cliff_catch_move_main_loop as *const () as _))
}
unsafe extern "C" fn cliff_catch_move_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CliffCatchMove_Main()
}
pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACH_WALL, attachwall_main);
    agent.status(Main, *FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE, cliff_catch_move_main);
}