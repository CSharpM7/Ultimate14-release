use crate::imports::imports_agent::*;

unsafe fn fuel_update(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object,ganon::instance::flag::SPECIAL_HI_START_RECHARGE) {
        let recharge = if fighter.is_grounded() {0.75} else {0.25};
        let charge = VarModule::get_float(fighter.battle_object,ganon::instance::float::SPECIAL_HI_START_FRAME)-recharge;
        VarModule::add_float(fighter.battle_object,ganon::instance::float::SPECIAL_HI_START_FRAME,-recharge);
        if charge <= 0.0 {
            VarModule::off_flag(fighter.battle_object,ganon::instance::flag::SPECIAL_HI_START_RECHARGE);
            VarModule::set_float(fighter.battle_object,ganon::instance::float::SPECIAL_HI_START_FRAME,0.0);
        }
        //renable up special after fuel
        else if charge <= (ganon::SPECIAL_HI_FUEL_MAX as f32 * 0.75) {
            if VarModule::is_flag(fighter.battle_object,ganon::instance::flag::SPECIAL_HI_COOLDOWN) {
                WorkModule::off_flag(fighter.module_accessor,*FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI_CONTINUOUS);
                VarModule::off_flag(fighter.battle_object,ganon::instance::flag::SPECIAL_HI_COOLDOWN);
            }
        }
    }
}
pub unsafe extern "C" fn ganon_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FighterInfo::get_common(fighter) {
        fuel_update(fighter);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main,  ganon_frame);
}