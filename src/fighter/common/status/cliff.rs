use crate::imports::imports_agent::*;

//Remove drift DI during unactionable frames
unsafe extern "C" fn fighter_cliff_robbed_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[STATUS_FRAME].get_f32();
    let max_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cliff_robbed_no_control_frame")) as f32;
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if frame < max_frame {
        let lr = PostureModule::lr(fighter.module_accessor);
        let speed_x: f32 = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("cliff_robbed_speed_x"));
        sv_kinetic_energy!(
            clear_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL
        );
        KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            speed_x*-lr,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else {
        if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            KineticModule::enable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::unable_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_STOP);
            let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor,*KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST,
                sum_speed_x,
                0.0,
                0.0,
                0.0,
                0.0
            );
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                sum_speed_x,
                0.0
            );
            let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
            let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
            let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
            let accel_x_mul = 0.5;
            let speed_x_max_mul = 1.0;

            sv_kinetic_energy!(
                controller_set_accel_x_mul,
                fighter,
                air_accel_x_mul * accel_x_mul
            );
            sv_kinetic_energy!(
                controller_set_accel_x_add,
                fighter,
                air_accel_x_add * accel_x_mul
            );
        }
    }
    0.into()
}


pub fn install(agent: &mut smashline::Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_CLIFF_ROBBED, fighter_cliff_robbed_exec);
}