use crate::imports::imports_agent::*;

unsafe extern "C" fn special_n_hold_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let cooldown = if is_kirby && false {VarModule::get_int(fighter.battle_object,kirby::instance::float::LUCAS_COOLDOWN)}
    else {WorkModule::get_int(fighter.module_accessor,lucas::SPECIAL_N_COOLDOWN)};
    //let factor = 1.0-((cooldown as f32)/(lucas::SPECIAL_N_COOLDOWN_MAX as f32));
    WorkModule::set_int(fighter.module_accessor,cooldown,lucas::SPECIAL_N_COOLDOWN_STATUS);
    let cooldown = WorkModule::get_int(fighter.module_accessor,lucas::SPECIAL_N_COOLDOWN_STATUS);

    let new_val = lerp(cooldown as f32,lucas::SPECIAL_N_COOLDOWN_MAX as f32,0.5) as i32;
    if is_kirby && false {VarModule::set_int(fighter.battle_object,kirby::instance::float::LUCAS_COOLDOWN,new_val);}
    else {WorkModule::set_int(fighter.module_accessor,new_val,lucas::SPECIAL_N_COOLDOWN);}

    println!("Lucas charge: {cooldown} > {new_val}");
    0.into()
}

unsafe extern "C" fn special_n_situation_changed(fighter: &mut L2CFighterCommon, init: bool) {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);

    let mot_g = hash40("special_n_hold");//WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
    let mot_a = hash40("special_air_n_hold");//WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
    let mot = if fighter.is_grounded() {Hash40::new_raw(mot_g)} else {Hash40::new_raw(mot_a)};

    if !init {
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_LUCAS_AIR_STOP_SPECIAL_N.into());
        let correct = if fighter.is_grounded() {*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK} else {*GROUND_CORRECT_KIND_AIR};
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(correct));
        if !is_kirby {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, mot, -1.0, 1.0, 0.0, false, false);
        }
        else {
            FighterMotionModuleImpl::change_motion_inherit_frame_kirby_copy(fighter.module_accessor,mot,-1.0,1.0, 0.0, false, false);
        }
    }
    else {
        if !is_kirby {
            MotionModule::change_motion(fighter.module_accessor, mot, 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            FighterMotionModuleImpl::change_motion_kirby_copy(fighter.module_accessor,mot,0.0, 1.0, false, 0.0, false, false);
        }
    }
}

unsafe extern "C" fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("time"));
    let nobang_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("nobang_time"));
    WorkModule::set_int(fighter.module_accessor, time,  *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_TIME);
    WorkModule::set_int(fighter.module_accessor, nobang_time,  *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_NOBANG_TIME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_MOT_CHANGE);

    /*NEW*/
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let cooldown = if is_kirby && false {VarModule::get_int(fighter.battle_object,kirby::instance::float::LUCAS_COOLDOWN)}
    else {WorkModule::get_int(fighter.module_accessor,lucas::SPECIAL_N_COOLDOWN)};
    //let factor = 1.0-((cooldown as f32)/(lucas::SPECIAL_N_COOLDOWN_MAX as f32));
    WorkModule::set_int(fighter.module_accessor,cooldown,lucas::SPECIAL_N_COOLDOWN_STATUS);

    let new_val = lerp(cooldown as f32,lucas::SPECIAL_N_COOLDOWN_MAX as f32,0.5) as i32;
    if is_kirby && false {VarModule::set_int(fighter.battle_object,kirby::instance::float::LUCAS_COOLDOWN,new_val);}
    else {WorkModule::set_int(fighter.module_accessor,new_val,lucas::SPECIAL_N_COOLDOWN);}

    println!("Cooldown: {cooldown} > {new_val}");
    /* */

	//WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_KIND);
	//WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold") as i64, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MOT_AIR_KIND);
	special_n_situation_changed(fighter,true);


    if !StopModule::is_stop(fighter.module_accessor) {
        special_n_hold_sub_status(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr( special_n_hold_sub_status as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_hold_main_loop as *const () as _))
}



unsafe extern "C" fn special_n_hold_sub_control(fighter: &mut L2CFighterCommon) {
    //Spawn if needed//
    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE) 
    && !WorkModule::is_flag(fighter.module_accessor,*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_ALREADY_GENERATED)
    {
        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_ALREADY_GENERATED);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE, false, -1);
    }
    if !WorkModule::is_flag(fighter.module_accessor,*FIGHTER_LUCAS_STATUS_SPECIAL_N_FLAG_ALREADY_GENERATED) {
        return;
    }

    //Detonate//
    let has_article = ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE);
    let min_countdown = WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_NOBANG_TIME, 0);
    let max_countdown = WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_TIME, 0);

    let is_min = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_NOBANG_TIME) <= 0;
    let is_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_TIME) <= 0;

    let is_button_off = ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
    if has_article {
        let shoot = is_max || (is_min && is_button_off);
        if shoot {
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), is_button_off);
            return;
        }
    }
}
unsafe extern "C" fn special_n_hold_sub_status(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if arg.get_bool() {
        special_n_hold_sub_control(fighter);
    }
    if !fighter.is_grounded() {
        let y_time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME);
        if y_time > 0 {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_STOP_Y_TIME);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        } 
        else {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }

    0.into()
}
unsafe extern "C" fn special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {

    let nobang_time = WorkModule::get_int(fighter.module_accessor,*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_NOBANG_TIME);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_PK_FREEZE) && nobang_time <= 0
    {
        let is_max = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_TIME) <= 0;
        let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
        let status_fire = if is_kirby {FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_FIRE} else {FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE};
        let status_end = if is_kirby {FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_END} else {FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_END};
        let next_status = if is_max {status_fire} else {status_end};
        fighter.change_status(next_status.into(), false.into());
        return 1.into();
    }

    let mut wait_mtrans_kind = WorkModule::get_int(fighter.module_accessor,*FIGHTER_LUCAS_STATUS_SPECIAL_N_WORK_INT_WAIT_MTRANS_KIND);
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        special_n_situation_changed(fighter,false);
    }

    0.into()
}

unsafe extern "C" fn test(fighter: &mut L2CFighterCommon) -> L2CValue {
    let owner_team =  FighterUtil::get_team_color(fighter.module_accessor);
    let my_entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    
    println!("Me {my_entry_id}: {owner_team}");
    let entry_count = lua_bind::FighterManager::entry_count(singletons::FighterManager());
    for i in 0..entry_count {
        let other_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i));
        if sv_battle_object::is_active((*other_boma).battle_object_id) {
            let other_entry_id = WorkModule::get_int(other_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
            //if my_entry_id == other_entry_id { continue; }
            let team_color = FighterUtil::get_team_color(&mut *other_boma);
            let team_no = TeamModule::hit_team_no(&mut *other_boma);
            //let effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
            println!("({other_entry_id}: {team_color} ({team_no})");
        }
    }
    return 0.into();
}
unsafe extern "C" fn special_n_fire_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;

    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_FIRE} else {*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE};
    let original: L2CValue = 0.into();//smashline::original_status(Init, fighter, original_status)(fighter);

    WorkModule::set_float(fighter.module_accessor,0.0,lucas::SPECIAL_N_HEAL_FRAME);
    return original;
}

unsafe extern "C" fn special_n_fire_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;

    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_FIRE} else {*FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE};
    let original: L2CValue = 0.into();//smashline::original_status(Exec, fighter, original_status)(fighter);

    let heal_frame = WorkModule::get_float(fighter.module_accessor,lucas::SPECIAL_N_HEAL_FRAME);
    if SearchModule::is_search(fighter.module_accessor,0) && heal_frame > 0.0 {
        let size = WorkModule::get_float(fighter.module_accessor, lucas::SPECIAL_N_HEAL_SIZE);
        SearchModule::set_size(fighter.module_accessor, 0, size);

        let frame = MotionModule::frame(fighter.module_accessor);
        if frame >= heal_frame{
            SearchModule::clear_all(fighter.module_accessor);
        }
    }
    return original;
}

pub fn install(fighter: &mut smashline::Agent) {
    fighter.status(Main, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_HOLD, special_n_hold_main);
    fighter.status(Init, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE, special_n_fire_init);
    fighter.status(Exec, *FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE, special_n_fire_exec);
    Agent::new("kirby")
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_HOLD, special_n_hold_main)
    .status(Init, *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_FIRE, special_n_fire_init)
    .status(Exec, *FIGHTER_KIRBY_STATUS_KIND_LUCAS_SPECIAL_N_FIRE, special_n_fire_exec)
    .install();
}