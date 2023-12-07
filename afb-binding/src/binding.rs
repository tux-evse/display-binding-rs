/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 */

use crate::prelude::*;
use afbv4::prelude::*;
use lvgl_gui::prelude::*;
use serde::{Deserialize, Serialize};

pub(crate) fn to_static_str(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

AfbDataConverter!(api_arg_subscribe, QuerySubscribe);
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(tag = "action")]
pub(crate) enum QuerySubscribe {
    #[default]
    SUBSCRIBE,
    UNSUBSCRIBE,
}

AfbDataConverter!(api_arg_switch, QuerySwitch);
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(tag = "action")]
pub(crate) enum QuerySwitch {
    #[default]
    ON,
    OFF,
}


fn json_to_color(jcolor: JsoncObj) -> Result<LvglColor, AfbError> {
    let red = jcolor.get::<u32>("red")?;
    let blue = jcolor.get::<u32>("blue")?;
    let green = jcolor.get::<u32>("green")?;

    Ok(LvglColor::rvb(red as u8, green as u8, blue as u8))
}

// Binding init callback started at binding load time before any API exist
// -----------------------------------------
pub fn binding_init(rootv4: AfbApiV4, jconf: JsoncObj) -> Result<&'static AfbApi, AfbError> {
    // add binding custom converter
    api_arg_subscribe::register()?;
    api_arg_switch::register()?;

    let uid = if let Ok(value) = jconf.get::<String>("uid") {
        to_static_str(value)
    } else {
        "display-lvgl"
    };

    let api = if let Ok(value) = jconf.get::<String>("api") {
        to_static_str(value)
    } else {
        uid
    };

    let info = if let Ok(value) = jconf.get::<String>("info") {
        to_static_str(value)
    } else {
        ""
    };

    afb_log_msg!(
        Info,
        rootv4,
        "Binding starting uid:{} api:{} info:{}",
        uid,
        api,
        info
    );

    let permission = if let Ok(value) = jconf.get::<String>("permission") {
        AfbPermission::new(to_static_str(value))
    } else {
        AfbPermission::new("acl:display:client")
    };

    let mut display = if let Ok(jvalue) = jconf.get::<JsoncObj>("display") {
        let x_res = jvalue.get::<u32>("x_res")? as i16;
        let y_res = jvalue.get::<u32>("y_res")? as i16;
        let ratio = jvalue.get::<u32>("ratio")?;

        DisplayHandle::create(x_res, y_res, ratio)
    } else {
        return Err(AfbError::new(
            "display-config-fail",
            "mandatory 'display' config missing",
        ));
    };

    let value = "/home/ronan/IoT/Valeo/lvgl/tux-evse-webapp/src/assets/images/tux_evsex150.png";
    LvglImage::new("tux-evse", value,0,0);

    // check theme and provide default if needed
    if let Ok(jvalue) = jconf.get::<JsoncObj>("theme") {
        let dark = jvalue.get::<bool>("dark")?;
        let primary = json_to_color(jvalue.get::<JsoncObj>("primary")?)?;
        let secondary = json_to_color(jvalue.get::<JsoncObj>("secondary")?)?;
        display.set_theme(primary, secondary, dark, LvglMkFont::std_14());
    } else {
        let primary = LvglColor::palette(LvglPalette::LIGHT_BLUE);
        let secondary = LvglColor::palette(LvglPalette::BLUE_GREY);
        // Fulup TBD apply a correct theme
        display.set_theme(primary, secondary, true, LvglMkFont::std_14());
    }

    // create backend API
    let api = AfbApi::new(api).set_info(info).set_permission(permission);
    register_verbs(api, &mut display)?;

    // lock config in ram to avoid lvgl to free memory
    Box::leak(Box::new(display));

    Ok(api.finalize()?)
}

// register binding within libafb
AfbBindingRegister!(binding_init);
