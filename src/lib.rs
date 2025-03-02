#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused,
    unused_macros,
    warnings
)]
#![deny(
    deprecated
)]


//#[macro_use]
//extern crate lazy_static;

pub use skyline::libc::c_char;

mod fighter;
mod custom_vars;
pub mod util;
//pub mod ext;
pub mod vars;
pub mod imports;

extern "C" {
    fn change_version_string(arg: u64, string: *const c_char);
}

#[skyline::hook(replace = change_version_string)]
fn change_version_string_hook(arg: u64, string: *const c_char) {
    let original_str = unsafe { skyline::from_c_str(string) };
    if original_str.contains("Ver.") {
        let new_str = format!(
            "Ver. 14.0.0\0",
        );
        call_original!(arg, skyline::c_str(&new_str))
    } else {
        call_original!(arg, string)
    }
}
fn show_error(code: u32, message: &str, details: &str) {
    use skyline::nn::{err, settings};

    let mut message_bytes = String::from(message).into_bytes();
    let mut details_bytes = String::from(details).into_bytes();

    if message_bytes.len() >= 2048 {
        message_bytes.truncate(2044);
        message_bytes.append(&mut String::from("...\0").into_bytes());
    }

    if details_bytes.len() >= 2048 {
        details_bytes.truncate(2044);
        details_bytes.append(&mut String::from("...\0").into_bytes());
    }

    unsafe {
        let error = err::ApplicationErrorArg::new_with_messages(
            code,
            core::str::from_utf8(&message_bytes).unwrap().as_bytes().as_ptr(),
            core::str::from_utf8(&details_bytes).unwrap().as_bytes().as_ptr(),
            &settings::LanguageCode_Make(settings::Language_Language_English)
        );

        err::ShowApplicationError(&error);
    }
}
pub fn quick_validate_install() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>"
            }
        };

        let err_msg = format!("thread has panicked at '{}', {}", msg, location);
        show_error(
            1991,
            "Ultimate 14 has panicked! Open details for more info. \n",
            err_msg.as_str()
        );
    }));

    let has_param_config = std::fs::metadata("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libparam_config.nro").is_ok();
    let has_nro_hook = std::fs::metadata("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libnro_hook.nro").is_ok();
    let has_smashline = std::fs::metadata("sd:/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/libsmashline_plugin.nro").is_ok();
    let has_skyline = std::fs::metadata("sd:/atmosphere/contents/01006a800016e000/exefs/").is_ok();
    let toreturn = has_param_config && has_nro_hook && has_smashline && has_skyline;

    if !has_param_config {
        panic!("Param Config not found! Please install libparam_config.nro to the plugin folder!");
    }
    if !has_nro_hook {
        panic!("NRO Hook not found! Please install libnro_hook.nro to the plugin folder!");
    }
    if !has_smashline {
        panic!("Smashline 2 not found! Please install smashline_plugin.nro to the plugin folder!");
    }
    if !has_skyline {
        panic!("Skyline not found! Please install skyline before using Rash");
    }
}

#[skyline::main(name = "ultimate14")]
pub fn main() {
    println!("[smashline_14::main] Loading...");

    #[cfg(feature = "dev")]{
        fighter::smashline_install();
        return;
    }

    quick_validate_install();
    custom_vars::install();
    skyline::install_hooks!(change_version_string_hook);

    fighter::install();

    println!("[smashline_14::main] Loaded!");
}