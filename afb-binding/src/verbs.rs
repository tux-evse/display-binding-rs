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

struct WidgetEvtCtx {
    event: &'static AfbEvent,
}

impl LvglHandler for WidgetEvtCtx {
    fn callback(&self, uid: &'static str, event: u32) {
        let info = format!("{{'uid':{}, 'event':{}}}", uid, event);
        println!("*** {} ***", info);
        self.event.push(&info);
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
    match args.get::<&ApiAction>(0)? {
        ApiAction::SUBSCRIBE => {
            ctx.event.subscribe(rqt)?;
        }
        ApiAction::UNSUBSCRIBE => {
            ctx.event.unsubscribe(rqt)?;
        }
    }
    rqt.reply(AFB_NO_DATA, 0);
    Ok(())
}

struct InfoVerbCtx {
    widget: &'static LvglWidget,
}
AfbVerbRegister!(InfoVerb, info_verb_cb, InfoVerbCtx);
fn info_verb_cb(rqt: &AfbRequest, args: &AfbData, ctx: &mut InfoVerbCtx) -> Result<(), AfbError> {
    let text = args.get::<String>(0)?;
    ctx.widget.set_text(text.as_str());
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

    let widget = match display.get_by_uid("Text-Area") {
        Some(value) => value,
        None => {
            return Err(AfbError::new(
                "verb-info-widget",
                "no 'Text-Area' widget found in panel".to_string(),
            ))
        }
    };
    let info = AfbVerb::new("set_info")
        .set_info("change text info zone")
        .set_callback(Box::new(InfoVerbCtx { widget }))
        .finalize()?;

    api.add_verb(subscribe);
    api.add_verb(info);

    Ok(())
}
