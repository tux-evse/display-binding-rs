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
use typesv4::prelude::*;

use afbv4::prelude::*;

use display_lvgl_gui::prelude::*;
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

AfbDataConverter!(api_arg_switch, QueryOnOff);
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(tag = "action")]
pub(crate) enum QueryOnOff {
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

pub struct ApiConfig {
    pub engy_api: &'static str,
}

// wait until both apis (iso+slac) to be ready before trying event subscription
struct ApiUserData {
    engy_api: &'static str,
}

impl AfbApiControls for ApiUserData {
    fn config(&mut self, api: &AfbApi, config: JsoncObj) -> Result<(), AfbError> {
        Ok(()) // returning -1 will abort binder process
    }

    // the API is created and ready. At this level user may subcall api(s) declare as dependencies
    fn start(&mut self, api: &AfbApi) -> Result<(), AfbError> {
        afb_log_msg!(
            Notice,
            api,
            "subscribing charging_api api:{}",
            self.engy_api
        );

        AfbSubCall::call_sync(api, self.engy_api, "volts", "{'action':'subscribe'}")?;
        AfbSubCall::call_sync(api, self.engy_api, "energy", "{'action':'subscribe'}")?;
        AfbSubCall::call_sync(api, self.engy_api, "amps", "{'action':'subscribe'}")?;
        AfbSubCall::call_sync(api, self.engy_api, "power", "{'action':'subscribe'}")?;
        AfbSubCall::call_sync(api, self.engy_api, "adps", "{'action':'subscribe'}")?;

        afb_log_msg!(Notice, None, "subscribing charging_api done ");
        Ok(())
    }

    // mandatory unsed declaration
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

// Binding init callback started at binding load time before any API exist
// -----------------------------------------
pub fn binding_init(rootv4: AfbApiV4, jconf: JsoncObj) -> Result<&'static AfbApi, AfbError> {
    // add binding custom converter
    api_arg_subscribe::register()?;
    api_arg_switch::register()?;

    // add binding custom converter
    types_register()?;
    engy_registers()?;
    
    let uid = if let Ok(value) = jconf.get::<String>("uid") {
        to_static_str(value)
    } else {
        "display"
    };

    let api_name = if let Ok(value) = jconf.get::<String>("api") {
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
        api_name,
        info
    );

    let permission = if let Ok(value) = jconf.get::<String>("permission") {
        AfbPermission::new(to_static_str(value))
    } else {
        AfbPermission::new("acl:display:client")
    };

    let mut display = match jconf.get::<JsoncObj>("display") {
        Ok(jvalue) => {
            let x_res = jvalue.get::<u32>("x_res")?;
            let y_res = jvalue.get::<u32>("y_res")?;
            let ratio = jvalue.get::<u32>("ratio")?;

            DisplayHandle::create(x_res as i16, y_res as i16, ratio)
        }
        Err(_error) => {
            return Err(AfbError::new(
                "display-config-fail",
                "mandatory 'display' config missing",
            ));
        }
    };

    if let Ok(value) = jconf.get::<String>("logo") {
        LvglImage::new(display.get_root(), "tux-evse", value.as_str(), 0, 0);
    }

    // check theme and provide default if needed
    if let Ok(jvalue) = jconf.get::<JsoncObj>("theme") {
        let dark = jvalue.get::<bool>("dark")?;
        let primary = json_to_color(jvalue.get::<JsoncObj>("primary")?)?;
        let secondary = json_to_color(jvalue.get::<JsoncObj>("secondary")?)?;
        display.set_theme(primary, secondary, dark, LvglMkFont::std_14());
    } else {
        let primary = LvglColor::LIGHT_BLUE();
        let secondary = LvglColor::BLUE_GREY();
        // Fulup TBD apply a correct theme
        display.set_theme(primary, secondary, false, LvglMkFont::std_14());
    }

    let engy_api = if let Ok(value) = jconf.get::<String>("engy_api") {
        to_static_str(value)
    } else {
        return Err(AfbError::new(
            "binding-mgr-engy-config",
            "engy_api micro service api SHOULD be defined",
        ));
    };

    let api_config = ApiConfig { engy_api };

    // create backend API
    // --------------------------------------------------------
    let api = AfbApi::new(api_name)
        .set_info(info)
        .set_permission(permission)
        .set_callback(Box::new(ApiUserData { engy_api }));

    register_verbs(api, &mut display, api_config)?;
    
    api.require_api(engy_api);

    Ok(api.finalize()?)
}

// register binding within libafb
AfbBindingRegister!(binding_init);
