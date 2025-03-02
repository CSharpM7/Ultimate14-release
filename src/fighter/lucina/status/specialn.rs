use crate::imports::imports_agent::*;
pub unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;

    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_MARTH_SPECIAL_N} else {*FIGHTER_STATUS_KIND_SPECIAL_N};
    let original: L2CValue = smashline::original_status(Main, fighter, original_status)(fighter);
    
    let charge_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("charge_frame_max")) as i32;
    WorkModule::set_int(fighter.module_accessor,charge_max, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_CHARGE_COUNT);
    if !is_kirby {
        ArticleModule::generate_article(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOW, false, -1);
        ArticleModule::change_status_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE);
        ArticleModule::generate_article(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW, false, -1);

        let article = ArticleModule::get_article(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW);
        let article_id = smash::app::lua_bind::Article::get_battle_object_id(article);
        WorkModule::set_int64(fighter.module_accessor, article_id as i64, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_TERM);
    }
    else {
        VarModule::set_int(fighter.battle_object,kirby::instance::int::LUCINA_ARROW,0);
        let copied_link = WorkModule::get_int(fighter.module_accessor,*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
        for entry_id in 0..8 {
            // get the active battle object id and add it to the list
            let id = get_active_battle_object_id_from_entry_id(entry_id).unwrap_or(*BATTLE_OBJECT_ID_INVALID as u32);
            let object = get_battle_object_from_id(id);
            if object.is_null() { continue; }
            let object = unsafe { &mut *object };
            let kind = smash::app::utility::get_kind(object) as i32;
            if kind != *FIGHTER_KIND_LUCINA { continue; }

            println!("Lucina!");
            /*
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, false, -1);
            ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW, *WN_LINK_BOW_STATUS_KIND_HAVE);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_BOW,true,ArticleOperationTarget(0)); */
    
            ArticleModule::generate_article(object.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW, false, -1);

            let article = ArticleModule::get_article(object.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW);
            let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_boma = sv_battle_object::module_accessor(article_id);

            println!("1: {article_id}");
            WorkModule::set_int(article_boma, fighter.battle_object_id as i32, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID);
            WorkModule::set_int(article_boma, fighter.battle_object_id as i32, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
            TeamModule::set_team(article_boma, TeamModule::team_no(fighter.module_accessor) as i32, false);

            link_constraint_to_owner(article_boma,Hash40::new("have"),Hash40::new("haver"));

            //WorkModule::set_int(fighter.module_accessor, article_id as i32, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_TERM);
            VarModule::set_int(fighter.battle_object,kirby::instance::int::LUCINA_ARROW, article_id as i32); 
            return 0.into();
        }
        println!("No Lucina?");
        WorkModule::set_int(fighter.module_accessor,*FIGHTER_KIND_KIRBY ,*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY);
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

pub unsafe extern "C" fn special_n_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    if !is_kirby {
        println!("Lucina end");
        ArticleModule::shoot_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    else {
        println!("Kirby end");
        let arrow_article = VarModule::get_int(fighter.battle_object,kirby::instance::int::LUCINA_ARROW) as u32;
        if arrow_article != 0 {
            println!("Shoot attempt {arrow_article}");
            if sv_battle_object::is_active((*arrow_article).battle_object_id) {
                let article_boma = sv_battle_object::module_accessor(arrow_article);
                StatusModule::change_status_request_from_script(article_boma,*WN_LINK_BOWARROW_STATUS_KIND_FLY,false);
            }
        }
    }

    //WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_CHARGE
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);

    let motion_g = if is_kirby {hash40("lucina_special_n_end")} else {hash40("special_n_end")};
    let motion_a = if is_kirby {hash40("lucina_special_air_n_end")} else {hash40("special_air_n_end")};
    WorkModule::set_int64(fighter.module_accessor, motion_g as i64, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_MOTION);
    WorkModule::set_int64(fighter.module_accessor, motion_a as i64, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_AIR_MOTION);
    fighter.sub_change_motion_by_situation(Hash40::new_raw(motion_g).into(), Hash40::new_raw(motion_a).into(), false.into());

    fighter.sub_set_ground_correct_by_situation(true.into());
    fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_FALL.into());

    return fighter.sub_shift_status_main(L2CValue::Ptr(special_n_end_main_loop as *const () as _));
}
pub unsafe extern "C" fn special_n_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        fighter.sub_set_ground_correct_by_situation(false.into());
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_FALL.into());
        let motion_g = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_MOTION);
        let motion_a = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_AIR_MOTION);
        fighter.sub_change_motion_by_situation(Hash40::new_raw(motion_g).into(), Hash40::new_raw(motion_a).into(), true.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    0.into()
}

pub unsafe extern "C" fn special_n_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
pub unsafe extern "C" fn remove_articles(fighter: &mut L2CFighterCommon) {
    ArticleModule::remove_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    //figure out if the arrow was shot or nah
    ArticleModule::remove_exist(fighter.module_accessor, lucina::GENERATE_ARTICLE_BOWARROW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("handfalchion"),true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("waistfalchion"),false);
}
pub unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    remove_articles(fighter);
    0.into()
}

pub unsafe extern "C" fn special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Main, fighter, *FIGHTER_KIRBY_STATUS_KIND_LUCINA_SPECIAL_N_LOOP)(fighter)
}

pub unsafe extern "C" fn kirby_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor,*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_MARTH {
        return smashline::original_status(Main, fighter, *FIGHTER_KIRBY_STATUS_KIND_MARTH_SPECIAL_N)(fighter);
    }
    else {
        special_n_main(fighter)
    }
}
pub unsafe extern "C" fn kirby_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor,*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_MARTH {
        println!("Marth??");
        return smashline::original_status(Main, fighter, *FIGHTER_KIRBY_STATUS_KIND_MARTH_SPECIAL_N_END)(fighter);
    }
    else {
        special_n_end_main(fighter)
    }
}
pub unsafe extern "C" fn kirby_end_max_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::get_int(fighter.module_accessor,*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_MARTH {
        println!("Marth??");
        return smashline::original_status(Main, fighter, *FIGHTER_KIRBY_STATUS_KIND_MARTH_SPECIAL_N_END_MAX)(fighter);
    }
    else {
        special_n_end_main(fighter)
    }
}
pub fn install(agent: &mut smashline::Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(Main, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, special_n_end_main);
    agent.status(Main, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX, special_n_end_main);
    
    agent.status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_exit);
    agent.status(Exit, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, special_n_exit);
    agent.status(Exit, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, special_n_exit);
    agent.status(Exit, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX, special_n_exit);

    agent.status(End, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, special_n_end);
    agent.status(End, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX, special_n_end);
    /*
    Agent::new("kirby")
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_LUCINA_SPECIAL_N, kirby_main)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_LUCINA_SPECIAL_N_END, special_n_end_main)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_LUCINA_SPECIAL_N_END_MAX, special_n_end_main)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_MARTH_SPECIAL_N_END, kirby_end_main)
    .status(Main, *FIGHTER_KIRBY_STATUS_KIND_MARTH_SPECIAL_N_END_MAX, kirby_end_max_main) 
    .install();*/
}