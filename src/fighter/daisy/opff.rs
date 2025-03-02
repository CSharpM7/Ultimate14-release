use crate::imports::imports_agent::*;

pub unsafe extern "C" fn daisy_frame(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FighterInfo::get_common(fighter) {
        if !WorkModule::is_flag(fighter.module_accessor,*FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT) {
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT);
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT_RAY_CHECK);
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    //::Agent::new("daisy")
        agent.on_line(Main, daisy_frame);
        //.install();
}