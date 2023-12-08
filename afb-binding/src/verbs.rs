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

pub(crate) fn register_verbs(
    api: &mut AfbApi,
    display: &mut DisplayHandle,
) -> Result<(), AfbError> {
    // global display API event
    let event = AfbEvent::new("widget");

    // build panel register display callback
    display
        .set_callback(Box::new(WidgetEvtCtx { event }))
        .draw_panel()
        .finalize();

    let subscribe = AfbVerb::new("event")
        .set_info("subscribe to widget event")
        .set_action("['SUBSCRIBE','UNSUBSCRIBE']")?
        .set_callback(Box::new(SubscribeEvtCtx { event }))
        .finalize()?;
    api.add_verb(subscribe);

    //create and register widget verbs (Warning type mismatch is only detected at runtime)
    verb_by_uid!(api, display, "Text-Area", LvglTextArea, TextCtx);
    verb_by_uid!(api, display, "Meter", LvglMeter, MeterCtx);
    verb_by_uid!(api, display, "Led-Green", LvglLed, LedCtx);
    verb_by_uid!(api, display, "Led-Red", LvglLed, LedCtx);
    verb_by_uid!(api, display, "Switch-1", LvglSwitch, SwitchCtx);
    verb_by_uid!(api, display, "Switch-2", LvglSwitch, SwitchCtx);
    verb_by_uid!(api, display, "Bar-1", LvglBar, BarCtx);
    verb_by_uid!(api, display, "Bar-2", LvglBar, BarCtx);
    verb_by_uid!(api, display, "Arc", LvglArc, ArcCtx);
    verb_by_uid!(api, display, "Pixmap-Button", LvglPixButton, NfcCtx);

    // register verb+event
    api.add_event(event);
    Ok(())
}
