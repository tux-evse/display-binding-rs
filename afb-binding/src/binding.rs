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
    pub chmgr_api: &'static str,
    pub auth_api: &'static str,
    pub dbus_api: &'static str,
}

// wait until both apis (iso+slac) to be ready before trying event subscription
struct ApiUserData {
    engy_api: &'static str,
    chmgr_api: &'static str,
    auth_api: &'static str,
    dbus_api: &'static str,
    auth_widget: &'static  LvglPixmap,
}

impl AfbApiControls for ApiUserData {
    fn config(&mut self, _api: &AfbApi, _config: JsoncObj) -> Result<(), AfbError> {
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

        AfbSubCall::call_sync(api, self.engy_api, "tension", "{'action':'subscribe'}")?;
        AfbSubCall::call_sync(api, self.engy_api, "energy", "{'action':'subscribe'}")?;
        AfbSubCall::call_sync(api, self.engy_api, "current", "{'action':'subscribe'}")?;
        AfbSubCall::call_sync(api, self.engy_api, "power", "{'action':'subscribe'}")?;
        /*Should be remove if unused
        if let Err(_msg_error) = AfbSubCall::call_sync(api, self.engy_api, "adsp", "{'action':'subscribe'}") {
            afb_log_msg!(Warning, api, "subscribing To adsp failed, linky missing");
        }
        */

        AfbSubCall::call_sync(api, self.auth_api, "subscribe", true)?;
        AfbSubCall::call_sync(api, self.chmgr_api, "subscribe", true)?;

        AfbSubCall::call_sync(api, self.dbus_api, "subscribe_nfc", true)?;

        let api_config = ApiConfig{ engy_api:self.engy_api , chmgr_api:self.chmgr_api, auth_api:self.auth_api, dbus_api:self.dbus_api};

        init_display_value(api, self.auth_widget, api_config)?;

        afb_log_msg!(Notice, api, "subscribing charging_api done ");

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
    engy_registers()?;
    auth_registers()?;
    chmgr_registers()?;

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

            let mut display-lvgl = DisplayHandle::create(x_res as i16, y_res as i16, ratio);
            display-lvgl.set_rotation(lvgl::disp::Rotation::Rotate180);
            display-lvgl
            
        }
        Err(_error) => {
            return afb_error!(
                "display-config-fail",
                "mandatory 'display' config missing",
            );
        }
    };

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
        return afb_error!(
            "binding-mgr-engy-config",
            "engy_api micro service api SHOULD be defined",
        );
    };


    let chmgr_api = if let Ok(value) = jconf.get::<String>("chmgr_api") {
        to_static_str(value)
    } else {
        return afb_error!(
            "binding-mgr-chmgr-config",
            "chmgr_api micro service api SHOULD be defined",
        );
    };


    let auth_api = if let Ok(value) = jconf.get::<String>("auth_api") {
        to_static_str(value)
    } else {
        return afb_error!(
            "binding-mgr-auth-config",
            "auth_api micro service api SHOULD be defined",
        );
    };

    let dbus_api = if let Ok(value) = jconf.get::<String>("dbus_api") {
        to_static_str(value)
    } else {
        return afb_error!(
            "binding-dbus_api-config",
            "dbus_api micro service api SHOULD be defined",
        );
    };

    let api_config = ApiConfig { engy_api , chmgr_api, auth_api, dbus_api};
    
    // create backend API
    // --------------------------------------------------------
    let api = AfbApi::new(api_name)
        .set_info(info)
        .set_permission(permission);

    register_verbs(api, &mut display, api_config)?;

    let auth_widget = match display.get_by_uid("Pixmap-auth-status").downcast_ref::<LvglPixmap>() {
        Some(auth_widget) => auth_widget,
        None => {
            return afb_error!(
                "verb-info-widget",
                "no widget uid:{} type:{} found in panel",
                "Pixmap-auth-status",
                "LvglPixmap"
            )
        }
    };

    api.set_callback(Box::new(ApiUserData { engy_api, chmgr_api, auth_api, dbus_api, auth_widget}));
    
    api.require_api(engy_api);
    api.require_api(chmgr_api);
    api.require_api(auth_api);
    api.require_api(dbus_api);

    Ok(api.finalize()?)
}

// register binding within libafb
AfbBindingRegister!(binding_init);
