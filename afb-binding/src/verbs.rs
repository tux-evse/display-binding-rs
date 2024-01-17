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
use typesv4::prelude::*;

use std::cell::Cell;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

macro_rules! handler_by_uid {
    ($api: ident, $display:ident, $uid:literal, $apievt:ident, $pattern:literal, $widget:ty, $ctx_type: ident) => {
        let widget = match $display.get_by_uid($uid).downcast_ref::<$widget>() {
            Some(widget) => widget,
            None => {
                return afb_error!(
                    "verb-info-widget",
                    "no widget uid:{} type:{} found in panel",
                    $uid,
                    stringify!($widget)
                )
            }
        };
        let handler = AfbEvtHandler::new(widget.get_uid())
            .set_info(widget.get_info())
            .set_pattern(to_static_str(format!("{}/{}", $apievt, $pattern)))
            .set_callback(Box::new($ctx_type { widget }))
            .finalize()?;

        $api.add_evt_handler(handler);
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
fn event_get_callback(
    event: &AfbEventMsg,
    args: &AfbData,
    userdata: &mut EvtUserData,
) -> Result<(), AfbError> {
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

    let _value = match args.get::<JsoncObj>(0) {
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

struct MgrEvtEngyCtrl {
    widget: &'static LvglLabel,
}

struct MgrEvtChmgrCtrl {
    widget_charge: &'static LvglPixmap,
    widget_plug_status: &'static LvglPixmap,
}

struct MgrEvtAuthCrl {
    widget: &'static LvglPixmap,
}

//------------------------------------------------------------------
struct PlugEvtCtrl {
    plug_pixmap: &'static LvglPixmap,
    switch_iso: &'static LvglSwitch,
    switch_pnc: &'static LvglSwitch,
    switch_v2g: &'static LvglSwitch,
    pixmap_start: &'static LvglPixButton,
}

AfbEventRegister!(MgrEvtEngyVerb, evt_nrj_cb, MgrEvtEngyCtrl);
fn evt_nrj_cb(
    event: &AfbEventMsg,
    args: &AfbData,
    ctx: &mut MgrEvtEngyCtrl,
) -> Result<(), AfbError> {
        let data = args.get::<&MeterDataSet>(0)?;
        ctx.widget.set_value(format!("{:.2}", (data.total as f64)/100.0).as_str());
        Ok(())
}

AfbEventRegister!(MgrEvtChmgrVerb, evt_chmgr_cb, MgrEvtChmgrCtrl);
fn evt_chmgr_cb(
    event: &AfbEventMsg,
    args: &AfbData,
    ctx: &mut MgrEvtChmgrCtrl,
) -> Result<(), AfbError> {
        afb_log_msg!(Notice, event, "-- evt_chmgr_cb event.");
        let data = args.get::<&ChargingMsg>(0)?;
        match data {
            ChargingMsg::Power(pdata) => {
                match pdata {
                    PowerRequest::Start => {
                        ctx.widget_charge.set_value(AssetPixmap::station_charging());
                    }
                    PowerRequest::Stop(i32) => {
                        ctx.widget_charge.set_value(AssetPixmap::station_completed());
                    }
                    _ => {
                        
                    }
                }
            }
            ChargingMsg::Plugged(sdata) => {
                match sdata {
                    PlugState::PlugIn => {
                        ctx.widget_plug_status.set_value(AssetPixmap::plug_connected_unlocked());
                    }
                    PlugState::Lock => {
                        ctx.widget_charge.set_value(AssetPixmap::station_pending_autho());
                        ctx.widget_plug_status.set_value(AssetPixmap::plug_connected_locked());
                    }
                    PlugState::Error => {
                        ctx.widget_charge.set_value(AssetPixmap::station_out_of_order());
                        ctx.widget_charge.set_value(AssetPixmap::plug_error());
                    }
                    PlugState::PlugOut => {
                        ctx.widget_charge.set_value(AssetPixmap::station_available());
                        ctx.widget_plug_status.set_value(AssetPixmap::plug_disconnected());
                    }
                    PlugState::Unknown => {
                        ctx.widget_plug_status.set_value(AssetPixmap::plug_unknow());
                    }
                    _ => {
                        
                    }
                }
            }
            _ => {
            }
        }
        Ok(())
}

AfbEventRegister!(MgrEvtAuthVerb, evt_auth_cb, MgrEvtAuthCrl);
fn evt_auth_cb(
    event: &AfbEventMsg,
    args: &AfbData,
    ctx: &mut MgrEvtAuthCrl,
) -> Result<(), AfbError> {
        afb_log_msg!(Notice, event, "-- evt_auth_cb event");
        let data = args.get::<&AuthMsg>(0)?;
        match data {
            AuthMsg::Done => {
                ctx.widget.set_value(AssetPixmap::nfc_done());
            }
            AuthMsg::Fail => {
                ctx.widget.set_value(AssetPixmap::nfc_fail());
            }
            AuthMsg::Pending => {
                ctx.widget.set_value(AssetPixmap::nfc_pending());
            }
            AuthMsg::Idle => {
                ctx.widget.set_value(AssetPixmap::nfc_idle());
            }
        };

        Ok(())
}


AfbEventRegister!(PlugEvtVerb, evt_plug_cb, PlugEvtCtrl);
fn evt_plug_cb(event: &AfbEventMsg, args: &AfbData, ctx: &mut PlugEvtCtrl) -> Result<(), AfbError> {
    afb_log_msg!(Notice, event, "-- evt_plug_cb event");
    match args.get::<&VehicleState>(0) {
        Ok(data) => {
            match data.plugged {
                PlugState2del::PlugIn => {
                    ctx.plug_pixmap
                        .set_value(AssetPixmap::plug_connected_unlocked());
                }
                PlugState2del::Lock => {
                    ctx.plug_pixmap
                        .set_value(AssetPixmap::plug_connected_locked());
                }
                PlugState2del::Error => {
                    ctx.plug_pixmap.set_value(AssetPixmap::plug_error());
                }
                PlugState2del::PlugOut => {
                    ctx.plug_pixmap.set_value(AssetPixmap::plug_disconnected());
                }
                PlugState2del::Unknown => {
                    ctx.plug_pixmap.set_value(AssetPixmap::plug_unknow());
                }
                _ => {
                    afb_log_msg!(Error, None, "-- plug invalid status");
                }
            }
            match data.power_request {
                PowerRequest2del::Start => {
                    ctx.pixmap_start.set_value(AssetPixmap::btn_start());
                    ctx.pixmap_start.set_disable(true);
                }
                PowerRequest2del::Stop => {
                    ctx.pixmap_start.set_value(AssetPixmap::btn_stop());
                    ctx.pixmap_start.set_disable(false);
                }
                _ => {
                    afb_log_msg!(Error, None, "-- power_request invalid status");
                }
            }
            match data.iso15118 {
                Iso15118State::Iso20 => {
                    ctx.switch_iso.set_value(true);
                    ctx.switch_pnc.set_value(false);
                    ctx.switch_v2g.set_value(false);
                }
                Iso15118State::Iso2 => {
                    ctx.switch_iso.set_value(false);
                    ctx.switch_pnc.set_value(true);
                    ctx.switch_v2g.set_value(false);
                }
                Iso15118State::Iec => {
                    ctx.switch_iso.set_value(false);
                    ctx.switch_pnc.set_value(false);
                    ctx.switch_v2g.set_value(true);
                }
                _ => {
                    afb_log_msg!(Error, None, "-- switch invalid status");
                }
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
    config: ApiConfig,
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
    //------------------------------------------------------------------
    let engy_api = config.engy_api;
    let chmgr_api = config.chmgr_api;
    let auth_api = config.auth_api;

    handler_by_uid!(
        api,
        display,
        "ChargeVoltsVal",
        engy_api,
        "tension",
        LvglLabel,
        MgrEvtEngyCtrl
    );

    handler_by_uid!(
        api,
        display,
        "ChargeEnergysVal",
        engy_api,
        "energy",
        LvglLabel,
        MgrEvtEngyCtrl
    );

    handler_by_uid!(
        api,
        display,
        "ChargeImpsVal",
        engy_api,
        "current",
        LvglLabel,
        MgrEvtEngyCtrl
    );

    handler_by_uid!(
        api,
        display,
        "ChargePowerVal",
        engy_api,
        "power",
        LvglLabel,
        MgrEvtEngyCtrl
    );

    handler_by_uid!(
        api,
        display,
        "ChargeAdpsVal",
        engy_api,
        "adsp",
        LvglLabel,
        MgrEvtEngyCtrl
    );

    let widget_charge = match display.get_by_uid("Pixmap-charge-status").downcast_ref::<LvglPixmap>() {
        Some(widget) => widget,
        None => {
            return afb_error!(
                "verb-info-widget",
                "no widget uid:{} type:{} found in panel",
                "Pixmap-charge-status",
                "LvglPixmap"
            )
        }
    };

    let widget_plug_status = match display
        .get_by_uid("Pixmap-connect-status")
        .downcast_ref::<LvglPixmap>()
    {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "Pixmap-connect-status",
                "no widget uid: Pixmap-connect-status  type:LvglPixmap found in panel",
            ))
        }
    };


    let handler = AfbEvtHandler::new("Charger_manager")
        .set_info("Charger manager")
        .set_pattern(to_static_str(format!("{}/{}",chmgr_api, "*")))
        .set_callback(Box::new(MgrEvtChmgrCtrl{ widget_charge, widget_plug_status }))
        .finalize()?;

    api.add_evt_handler(handler);

    handler_by_uid!(
        api,
        display,
        "Pixmap-auth-status",
        auth_api,
        "*",
        LvglPixmap,
        MgrEvtAuthCrl
    );

    //------------------------------------------------------------------
    /*
    let lv_charge_duration = match display
        .get_by_uid("ChargeDuration")
        .downcast_ref::<LvglLabel>()
    {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "conf-date-widget",
                "no widget uid: ChargeDuration  type:LvglLabel found in panel",
            ))
        }
    };
*/


    let lv_Switch_iso = match display
        .get_by_uid("Switch-iso")
        .downcast_ref::<LvglSwitch>()
    {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "Switch-iso",
                "no widget uid: Switch-iso type:LvglSwitch found in panel",
            ))
        }
    };

    let lv_Switch_pnc = match display
        .get_by_uid("Switch-pnc")
        .downcast_ref::<LvglSwitch>()
    {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "Switch-pnc",
                "no widget uid: Switch-pnc  type:LvglSwitch found in panel",
            ))
        }
    };

    let lv_Switch_v2g = match display
        .get_by_uid("Switch-v2g")
        .downcast_ref::<LvglSwitch>()
    {
        Some(widget) => widget,
        None => {
            return Err(AfbError::new(
                "Switch-v2g",
                "no widget uid: Switch-v2g  type:LvglSwitch found in panel",
            ))
        }
    };

    let lv_pixmap_start = match display
        .get_by_uid("Pixmap-start")
        .downcast_ref::<LvglPixButton>()
    {
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
        .set_callback(Box::new(TimerCtx { time, date }))
        .start()?;

    let _ = types_register();
    Ok(())
}
