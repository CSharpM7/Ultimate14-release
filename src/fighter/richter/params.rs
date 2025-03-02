use smash::{lib::lua_const::*, hash40};

pub fn install() {
    let slots: Vec<i32> = vec![-1];
    let kind = *FIGHTER_KIND_RICHTER;

    let mut param_ints: Vec<(u64,u64,i32)> = Vec::new();
    let mut param_floats: Vec<(u64,u64,f32)> = Vec::new();

    param_floats.push((hash40("run_speed_max"),0 as u64, 1.62));//+0.5
    param_floats.push((hash40("air_accel_x_mul"),0 as u64, 0.04)); //+0.02
    param_floats.push((hash40("air_accel_x_add"),0 as u64, 0.01));
    param_floats.push((hash40("air_speed_x_stable"),0 as u64, 1.0)); //+0.06
    param_floats.push((hash40("weight"),0 as u64, 95.0)); //-12

    for p in param_ints {
        param_config::update_int_2(kind, slots.clone(), p);
    }
    for p in param_floats {
        param_config::update_float_2(kind, slots.clone(), p);
    }
}