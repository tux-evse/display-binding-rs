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
use display_lvgl_gui::prelude::*;

use std::cell::Cell;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

macro_rules! verb_by_uid {
    ($api: ident, $display:ident, $uid:literal, $widget:ty, $ctx_type: ident) => {
        let widget = match $display.get_by_uid($uid).downcast_ref::<$widget>() {
            Some(widget) => widget,
            None => {
                return Err(AfbError::new(
                    "verb-info-widget",
                    format!("no widget uid:{} type:{} found in panel", $uid, stringify!($widget)),
                ))
            }
        };

            let verb = AfbVerb::new(widget.get_uid())
                .set_info(widget.get_info())
                .set_action(widget.get_action())?
                .set_callback(Box::new($ctx_type { widget }));

            $api.add_verb(verb)
    };
}

struct WidgetEvtCtx {
    event: &'static AfbEvent,
}

impl LvglHandler for WidgetEvtCtx {
    fn callback(&self, widget: &LvglWidget, uid: &'static str, event: &LvglEvent) {
        match widget {
            LvglWidget::Label(this) => {
                println!("button:{} get event:{:?}", uid, event);
                this.set_value("was pressed");
            }
            _ => {}
        }

        let info = format!("{{'uid':{}, 'event':{:?}}}", uid, event);
        println!("*** {} ***", info);
        self.event.push(info);
    }
}

struct SubscribeEvtCtx {
    event: &'static AfbEvent,
}

AfbVerbRegister!(SubscribeEvtVerb, subscribe_evt_cb, SubscribeEvtCtx);
fn subscribe_evt_cb(
    rqt: &AfbRequest,
    args: &AfbData,
    ctx: &mut SubscribeEvtCtx,
) -> Result<(), AfbError> {
    match args.get::<&QuerySubscribe>(0)? {
        QuerySubscribe::SUBSCRIBE => {
            ctx.event.subscribe(rqt)?;
        }
        QuerySubscribe::UNSUBSCRIBE => {
            ctx.event.unsubscribe(rqt)?;
        }
    }
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct TextCtx {
    widget: &'static LvglTextArea,
}
AfbVerbRegister!(InfoVerb, info_verb_cb, TextCtx);
fn info_verb_cb(rqt: &AfbRequest, args: &AfbData, ctx: &mut TextCtx) -> Result<(), AfbError> {
    let text = args.get::<String>(0)?;
    ctx.widget.set_value(text.as_str());
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct MeterCtx {
    widget: &'static LvglMeter,
}
AfbVerbRegister!(MeterVerb, meter_verb_cb, MeterCtx);
fn meter_verb_cb(rqt: &AfbRequest, args: &AfbData, ctx: &mut MeterCtx) -> Result<(), AfbError> {
    let value = args.get::<i32>(0)?;
    ctx.widget.set_value(value);
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct ArcCtx {
    widget: &'static LvglArc,
}
AfbVerbRegister!(ArcVerb, arc_verb_cb, ArcCtx);
fn arc_verb_cb(rqt: &AfbRequest, args: &AfbData, ctx: &mut ArcCtx) -> Result<(), AfbError> {
    let value = args.get::<i32>(0)?;
    ctx.widget.set_value(value);
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct BarCtx {
    widget: &'static LvglBar,
}
AfbVerbRegister!(BarVerb, bar_verb_cb, BarCtx);
fn bar_verb_cb(rqt: &AfbRequest, args: &AfbData, ctx: &mut BarCtx) -> Result<(), AfbError> {
    let value = args.get::<i32>(0)?;
    ctx.widget.set_value(value);
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct NfcCtx {
    widget: &'static LvglPixButton,
}
AfbVerbRegister!(NfcVerb, ncf_verb_cb, NfcCtx);
fn ncf_verb_cb(rqt: &AfbRequest, args: &AfbData, ctx: &mut NfcCtx) -> Result<(), AfbError> {
    match args.get::<&QueryOnOff>(0)? {
        QueryOnOff::ON => {
            ctx.widget.set_value(AssetPixmap::nfc_on());
        }
        QueryOnOff::OFF => {
            ctx.widget.set_value(AssetPixmap::ethernet_on());
        }
    }
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct SwitchCtx {
    widget: &'static LvglSwitch,
}
AfbVerbRegister!(SwitchVerb, switch_verb_cb, SwitchCtx);
fn switch_verb_cb(rqt: &AfbRequest, args: &AfbData, ctx: &mut SwitchCtx) -> Result<(), AfbError> {
    match args.get::<&QueryOnOff>(0)? {
        QueryOnOff::ON => {
            ctx.widget.set_value(true);
        }
        QueryOnOff::OFF => {
            ctx.widget.set_value(false);
        }
    }
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct LedCtx {
    widget: &'static LvglLed,
}
AfbVerbRegister!(LedVerb, led_verb_cb, LedCtx);
fn led_verb_cb(rqt: &AfbRequest, args: &AfbData, ctx: &mut LedCtx) -> Result<(), AfbError> {
    match args.get::<&QueryOnOff>(0)? {
        QueryOnOff::ON => {
            ctx.widget.set_on(true);
        }
        QueryOnOff::OFF => {
            ctx.widget.set_on(false);
        }
    }
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct TimerCtx {
    time: &'static LvglLabel,
    date: &'static LvglLabel,
}
// Callback is called for each tick until decount>0
AfbTimerRegister!(TimerCtrl, timer_callback, TimerCtx);
fn timer_callback(_timer: &AfbTimer, _decount: u32, ctx: &mut TimerCtx) -> Result<(), AfbError> {
    ctx.time.set_value(get_time("%H:%M").unwrap().as_str());
    ctx.date.set_value(get_time("%D").unwrap().as_str());
    Ok(())
}

//------------------------------------------------------------------

struct UserCtxData {
    event: &'static AfbEvent,
    counter: Cell<u32>,
}

impl UserCtxData {
    fn incr_counter(&self) -> u32 {
        self.counter.set(self.counter.get() + 1);
        self.counter.get()
    }
}

struct EvtUserData {
    ctx: Arc<UserCtxData>,
}
AfbEventRegister!(EventGetCtrl, event_get_callback, EvtUserData);
fn event_get_callback(event: &AfbEventMsg, args: &AfbData, userdata: &mut EvtUserData)  -> Result<(), AfbError>  {
    // check request introspection
    let evt_uid = event.get_uid();
    let evt_name = event.get_name();
    let api_uid = event.get_api().get_uid();

    afb_log_msg!(
        Notice,
        event,
        "--callback evt={} name={} counter={} api={}",
        evt_uid,
        evt_name,
        userdata.ctx.incr_counter(),
        api_uid
    );

    let value = match args.get::<JsoncObj>(0) {
        Ok(argument) => {
            afb_log_msg!(Info, event, "Got valid jsonc object argument={}", argument);
            argument
        }
        Err(error) => {
            afb_log_msg!(Error, event, "hoop invalid json argument {}", error);
            JsoncObj::from("invalid json input argument")
        }
    };
    Ok(())
}
//------------------------------------------------------------------

struct MgrEvtCtrl {
    charge_total:  &'static LvglLabel,
    charge_duration:  &'static LvglLabel,
    charge_power:  &'static LvglLabel,
}

struct PlugEvtCtrl {
    plug_pixmap:  &'static LvglPixmap,
    switch_iso: &'static LvglSwitch,
    switch_pnc: &'static LvglSwitch,
    switch_v2g: &'static LvglSwitch,
    pixmap_start: &'static LvglPixButton
}

/*
AfbDataConverter!(vehicle_state_data, VehicleCurrentState);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", tag = "action")]
struct VehicleCurrentState {
    pub energy: f32,
}
*/

/*
AfbDataConverter!(plug_state_data, PlugStateData);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", tag = "action")]
struct PlugStateData {
    pub vehicleplugstatus: plug_state,
}
*/
/* 
pub fn types_register() -> Result<(),AfbError> {
    vehicle_state_data::register()?;
    //plug_state_data::register()?;
    Ok(())
}
*/
AfbEventRegister!(MgrEvtVerb, evt_nrj_cb, MgrEvtCtrl);
fn evt_nrj_cb(event: &AfbEventMsg, args: &AfbData, ctx: &mut MgrEvtCtrl) -> Result<(), AfbError> {
    match args.get::<&ChargerNrj>(0){
        Ok(data) => {
            //afb_log_msg!(Debug, event, "-- energy data:{}", data.energy );

            ctx.charge_total.set_value(format!("{}", data.energy).as_str());
            ctx.charge_duration.set_value(format!("{}", data.duration).as_str());
            ctx.charge_power.set_value(format!("{}", data.charge_power).as_str());
        }
        Err(_) => {
            afb_log_msg!(Error, event, "-- energy invalid data");
        }
    };
    Ok(())
}

AfbEventRegister!(PlugEvtVerb, evt_plug_cb, PlugEvtCtrl);
fn evt_plug_cb(event: &AfbEventMsg, args: &AfbData, ctx: &mut PlugEvtCtrl) -> Result<(), AfbError> {
    afb_log_msg!(Notice, event, "-- evt_plug_cb event");
    match args.get::<&VehicleState>(0){
        Ok(data) => {
            match data.plugged {
                PlugState::PlugIn => {ctx.plug_pixmap.set_value(AssetPixmap::plug_connected_unlocked());},
                PlugState::Lock => {ctx.plug_pixmap.set_value(AssetPixmap::plug_connected_locked());},
                PlugState::Error => {ctx.plug_pixmap.set_value(AssetPixmap::plug_error());},
                PlugState::PlugOut => {ctx.plug_pixmap.set_value(AssetPixmap::plug_disconnected());},
                PlugState::Unknown => {ctx.plug_pixmap.set_value(AssetPixmap::plug_unknow());},
                _ => {afb_log_msg!(Error, None, "-- plug invalid status");},
            }
            match data.power_request {
                PowerRequest::Start => {
                    ctx.pixmap_start.set_value(AssetPixmap::btn_start());
                    ctx.pixmap_start.set_disable(true);
                },
                PowerRequest::Stop => {
                    ctx.pixmap_start.set_value(AssetPixmap::btn_stop());
                    ctx.pixmap_start.set_disable(false);
                },
                _ => {afb_log_msg!(Error, None, "-- power_request invalid status");},
            }
            match data.iso15118 {
                Iso15118State::Iso20 => {
                    ctx.switch_iso.set_value(true);
                    ctx.switch_pnc.set_value(false);
                    ctx.switch_v2g.set_value(false);
                },
                Iso15118State::Iso2 => {
                    ctx.switch_iso.set_value(false);
                    ctx.switch_pnc.set_value(true);
                    ctx.switch_v2g.set_value(false);
                },
                Iso15118State::Iec => {
                    ctx.switch_iso.set_value(false);
                    ctx.switch_pnc.set_value(false);
                    ctx.switch_v2g.set_value(true);
                },
                _ => {afb_log_msg!(Error, None, "-- switch invalid status");},
            }
        }
        Err(_) => {
            afb_log_msg!(Error, event, "-- plug invalid format data");
        }
    }
    Ok(())
}

pub(crate) fn register_verbs(
    api: &mut AfbApi,
    display: &mut DisplayHandle,
    config: ApiConfig
) -> Result<(), AfbError> {
    // global display API event
    let event = AfbEvent::new("widget");

    // build panel register display callback
    display
        .set_callback(Box::new(WidgetEvtCtx { event }))
        .draw_panel()
        .finalize();

    //------------------------------------------------------------------

    let time = match display.get_by_uid("time").downcast_ref::<LvglLabel>() {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "conf-time-widget",
                "no widget uid: time  type:LvglLabel found in panel",
            ))
        }
    };
    let date = match display.get_by_uid("date").downcast_ref::<LvglLabel>() {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "conf-date-widget",
                "no widget uid: date  type:LvglLabel found in panel",
            ))
        }
    };

    let lv_charge_total = match display.get_by_uid("ChargeTotal").downcast_ref::<LvglLabel>() {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "conf-date-widget",
                "no widget uid: ChargeTotal  type:LvglLabel found in panel",
            ))
        }
    };
    let lv_charge_power = match display.get_by_uid("BatConso").downcast_ref::<LvglLabel>() {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "conf-date-widget",
                "no widget uid: BatConso  type:LvglLabel found in panel",
            ))
        }
    };
    
    let lv_charge_duration = match display.get_by_uid("ChargeDuration").downcast_ref::<LvglLabel>() {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "conf-date-widget",
                "no widget uid: ChargeDuration  type:LvglLabel found in panel",
            ))
        }
    };
    


    let lv_plug_status = match display.get_by_uid("Pixmap-connect-status").downcast_ref::<LvglPixmap>() {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "Pixmap-connect-status",
                "no widget uid: Pixmap-connect-status  type:LvglPixmap found in panel",
            ))
        }
    };

    let lv_Switch_iso = match display.get_by_uid("Switch-iso").downcast_ref::<LvglSwitch>() {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "Switch-iso",
                "no widget uid: Switch-iso type:LvglSwitch found in panel",
            ))
        }
    };

    let lv_Switch_pnc = match display.get_by_uid("Switch-pnc").downcast_ref::<LvglSwitch>() {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "Switch-pnc",
                "no widget uid: Switch-pnc  type:LvglSwitch found in panel",
            ))
        }
    };

    let lv_Switch_v2g = match display.get_by_uid("Switch-v2g").downcast_ref::<LvglSwitch>() {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "Switch-v2g",
                "no widget uid: Switch-v2g  type:LvglSwitch found in panel",
            ))
        }
    };

    let lv_pixmap_start = match display.get_by_uid("Pixmap-start").downcast_ref::<LvglPixButton>() {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "Pixmap-start",
                "no widget uid: Pixmap-start  type:LvglPixButton found in panel",
            ))
        }
    };

    AfbTimer::new("clock-timer")
      .set_period(60000)
      .set_callback(Box::new(TimerCtx{time,date}))
      .start()?;


    let mgr_handle= AfbEvtHandler::new("mgr-nrj")
        .set_info("nrj manager binding")
        .set_pattern(to_static_str(format!("{}/evt-nrj-status", config.mgr_api)))
        .set_callback(Box::new(MgrEvtCtrl {
            charge_total: lv_charge_total,
            charge_duration: lv_charge_duration,
            charge_power: lv_charge_power,
        }))
        .finalize()?;

    let mgr_plug_handle= AfbEvtHandler::new("plug-status")
        .set_info("plug status binding")
        .set_pattern(to_static_str(format!("{}/evt-plug-status", config.mgr_api)))
        .set_callback(Box::new(PlugEvtCtrl {
            plug_pixmap: lv_plug_status,
            switch_iso: lv_Switch_iso,
            switch_pnc: lv_Switch_pnc,
            switch_v2g: lv_Switch_v2g,
            pixmap_start: lv_pixmap_start,
        }))
        .finalize()?;
  
    api.add_evt_handler(mgr_handle);
    api.add_evt_handler(mgr_plug_handle);
    let _ = types_register();
    Ok(())
}
