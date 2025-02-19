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
use std::cell::RefCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

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

struct TimerCtx {
    time: &'static LvglLabel,
    date: &'static LvglLabel,
}
// Callback is called for each tick until decount>0
fn timer_callback(_timer: &AfbTimer, _decount: u32, ctx_data: &AfbCtxData) -> Result<(), AfbError> {
    let ctx = ctx_data.get_ref::<TimerCtx>()?;
    ctx.time.set_value(get_time("%H:%M:%S").unwrap().as_str());
    ctx.date.set_value(get_time("%D").unwrap().as_str());

    Ok(())
}

pub struct ChargingTimerCtx {
    pub elapsed_time: &'static LvglLabel,
    pub start_charging_time: Cell<SystemTime>,
    pub stop_animation: Cell<bool>,
    pub is_charging: Cell<bool>,
    pub elapsed_time_val: RefCell<String>, // For session summary
}

fn charging_timer_cb(
    _timer: &AfbTimer,
    _decount: u32,
    ctx_data: &AfbCtxData,
) -> Result<(), AfbError> {
    let ctx = ctx_data.get_ref::<Arc<ChargingTimerCtx>>()?;
    if ctx.is_charging.get() {
        if let Ok(elapsed) = ctx.start_charging_time.get().elapsed() {
            let elapsed_seconds = elapsed.as_secs();
            let minutes = (elapsed_seconds % 3600) / 60;
            let seconds = elapsed_seconds % 60;
            let elapsed_str = format!("{:02}:{:02}", minutes, seconds);
            ctx.elapsed_time.set_value(&elapsed_str);
            // Update the value
            *ctx.elapsed_time_val.borrow_mut() = elapsed_str.clone();
        }
    } else {
        ctx.elapsed_time.set_value("00:00");
    }

    Ok(())
}

fn animation_timer_cb(
    _timer: &AfbTimer,
    _decount: u32,
    ctx_data: &AfbCtxData,
) -> Result<(), AfbError> {
    let ctx = ctx_data.get_ref::<Arc<MgrEvtChmgrCtrl>>()?;
    let charging_timer_ctx = &ctx.charging_timer_ctx;

    if !charging_timer_ctx.stop_animation.get() {
        if charging_timer_ctx.is_charging.get() {
            if let Ok(elapsed) = charging_timer_ctx.start_charging_time.get().elapsed() {
                let elapsed_seconds = elapsed.as_secs();
                if elapsed_seconds > 5 {
                    match ctx.charging_protocol.get() {
                        ChargingProtocol::PlugAndCharge => {
                            animate_pnc_flow(&ctx);
                        }
                        ChargingProtocol::Vehicle2Grid => animate_discharge_flow(&ctx),
                        _ => {
                            animate_flow(&ctx);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

//------------------------------------------------------------------

struct MgrEvtChmgrCtrl {
    widget_evse: &'static LvglPixmap,
    widget_progress: &'static LvglPixmap,
    widget_state_msg: &'static LvglPixmap,
    label_elapsed_time: &'static LvglLabel,
    pub charging_timer_ctx: Arc<ChargingTimerCtx>, // Shared context for managing the timer
    pub charging_summary_ctx: Arc<ChargingSummaryCtx>,
    pub charging_protocol: Cell<ChargingProtocol>,
    pub energy_ctx: Arc<MgrEvtEngyCtrl>,
}

pub struct ChargingSummaryCtx {
    pub label_summary_session: &'static LvglLabel,
    pub label_charging_protocol: &'static LvglLabel,
    pub label_charging_duration: &'static LvglLabel,
    pub label_summary_energy_delivered: &'static LvglLabel,
    pub label_total_session_cost: &'static LvglLabel,
    pub label_charging_protocol_val: &'static LvglLabel,
    pub label_charging_duration_val: &'static LvglLabel,
    pub label_energy_delivered_val: &'static LvglLabel,
    pub label_total_session_cost_val: &'static LvglLabel,
}

struct MgrEvtAuthCrl {
    widget_evse: &'static LvglPixmap,
    widget_progress: &'static LvglPixmap,
    widget_state_msg: &'static LvglPixmap,
}

//------------------------------------------------------------------
pub struct MgrEvtEngyCtrl {
    label_energy_delivered: &'static LvglLabel,
    label_current_power: &'static LvglLabel,
    pub energy_deliverd: RefCell<String>,
    pub total_cost: RefCell<String>,
}

fn evt_nrj_cb(
    _event: &AfbEventMsg,
    args: &AfbRqtData,
    ctx_data: &AfbCtxData,
) -> Result<(), AfbError> {
    let ctx = ctx_data.get_ref::<Arc<MgrEvtEngyCtrl>>()?;
    let data = args.get::<&MeterDataSet>(0)?;

    match data.tag {
        MeterTagSet::Energy => {
            // energy delivered
            let meter_data = format!("{:.2}", (data.total as f64) / 1000.0);
            ctx.label_energy_delivered.set_value(&meter_data);
            let energy_value_with_unit = format!("{:.2} kWh", (data.total as f64) / 1000.0);
            let cost_value_witn_unit = format!("{:.2} Euros", (data.total as f64) * 0.36 / 1000.0);
            // For session summary
            *ctx.energy_deliverd.borrow_mut() = energy_value_with_unit.clone();
            *ctx.total_cost.borrow_mut() = cost_value_witn_unit.clone();
        }
        MeterTagSet::Power => {
            // current power
            let meter_data = format!("{:.2}", (data.total as f64) / 1000000.0);
            ctx.label_current_power.set_value(&meter_data);
        }
        _ => {}
    }
    Ok(())
}

fn evt_chmgr_cb(
    event: &AfbEventMsg,
    args: &AfbRqtData,
    ctx_data: &AfbCtxData,
) -> Result<(), AfbError> {
    let ctx = ctx_data.get_ref::<Arc<MgrEvtChmgrCtrl>>()?;
    let charging_timer_ctx = &ctx.charging_timer_ctx;
    let charging_summary_ctx = &ctx.charging_summary_ctx;
    let energy_ctx = &ctx.energy_ctx;
    let data = args.get::<&ChargingMsg>(0)?;
    afb_log_msg!(Notice, event, "-- evt_chmgr_cb event:{:?}.", data);
    match data {
        ChargingMsg::ServiceStatus { name, status } => {
            afb_log_msg!(
                Notice,
                None,
                "ServiceStatus update: {} -> {:?}",
                name,
                status
            );
            match status {
                ServiceStatus::Ready => {
                    afb_log_msg!(Notice, None, "ServiceStatus::Ready");
                    ctx.widget_evse.set_value(AssetPixmap::evse_ready());
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_init());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_msg_ready());
                }
                ServiceStatus::Starting => {
                    afb_log_msg!(Notice, None, "ServiceStatus::Starting");
                }
                ServiceStatus::Stopping => {
                    afb_log_msg!(Notice, None, "ServiceStatus::Stopping");
                }
                ServiceStatus::Error => {
                    afb_log_msg!(Error, None, "ServiceStatus::Error");
                }
            }
        }
        ChargingMsg::Power(pdata) => {
            match pdata {
                PowerRequest::Start => {
                    afb_log_msg!(Notice, None, "Display::PowerRequest::Start");
                }
                PowerRequest::Charging(_value) => {
                    afb_log_msg!(Notice, None, "Display::PowerRequest::Charging");
                    // Start elapsed time
                    charging_timer_ctx.is_charging.set(true);
                    charging_timer_ctx
                        .start_charging_time
                        .set(SystemTime::now());
                }
                PowerRequest::Stop(_value) => {
                    afb_log_msg!(Notice, None, "Display::PowerRequest::Stop");
                    charging_timer_ctx.is_charging.set(false);
                    ctx.widget_evse.set_value(AssetPixmap::evse_simple());
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_complete());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_msg_complete_unplug());
                }
                PowerRequest::Idle => {
                    afb_log_msg!(Notice, None, "Display::PowerRequest::Idle");
                    ctx.widget_evse.set_value(AssetPixmap::evse_ready());
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_init());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_msg_ready());
                }
            }
        }
        ChargingMsg::Plugged(sdata) => {
            match sdata {
                PlugState::PlugIn => {
                    afb_log_msg!(Notice, None, "Display::PlugState::PlugIn");
                    // ctx.widget_plug_status.set_value(AssetPixmap::plug_connected_unlocked());
                    // ctx.widget_evse.set_value(AssetPixmap::evse_pnc_auth());
                    ctx.widget_evse.set_value(AssetPixmap::evse_pnc_auth());
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_pnc_auth());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_msg_init());
                }
                PlugState::Lock => {
                    afb_log_msg!(Notice, None, "Display::PlugState::Lock");
                }
                PlugState::Error => {
                    afb_log_msg!(Notice, None, "Display::PlugState::Error");
                    ctx.widget_evse.set_value(AssetPixmap::evse_simple());
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_plug_fail());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_msg_init());
                }
                PlugState::PlugOut => {
                    afb_log_msg!(Notice, None, "Display::PlugState::PlugOut");
                    // Add Session Summary
                    ctx.widget_evse.set_value(AssetPixmap::evse_simple());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_msg_blank());
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_complete());
                    // Session Summary
                    charging_summary_ctx
                        .label_summary_session
                        .set_value("Session Summary");
                    charging_summary_ctx
                        .label_charging_protocol
                        .set_value("Protocol used:");
                    charging_summary_ctx
                        .label_charging_duration
                        .set_value("Charging time:");
                    charging_summary_ctx
                        .label_summary_energy_delivered
                        .set_value("Energy delivered:");
                    charging_summary_ctx
                        .label_total_session_cost
                        .set_value("Total session cost:");

                    charging_summary_ctx
                        .label_charging_protocol_val
                        .set_value(ctx.charging_protocol.get().as_str());
                    charging_summary_ctx
                        .label_charging_duration_val
                        .set_value(&charging_timer_ctx.elapsed_time_val.borrow());
                    charging_summary_ctx
                        .label_energy_delivered_val
                        .set_value(&energy_ctx.energy_deliverd.borrow());
                    charging_summary_ctx
                        .label_total_session_cost_val
                        .set_value(&energy_ctx.total_cost.borrow());

                    thread::sleep(Duration::from_secs(15));
                    // Remove Session Summary
                    charging_summary_ctx.label_summary_session.set_value("");
                    charging_summary_ctx.label_charging_protocol.set_value("");
                    charging_summary_ctx.label_charging_duration.set_value("");
                    charging_summary_ctx
                        .label_summary_energy_delivered
                        .set_value("");
                    charging_summary_ctx.label_total_session_cost.set_value("");

                    charging_summary_ctx
                        .label_charging_protocol_val
                        .set_value("");
                    charging_summary_ctx
                        .label_charging_duration_val
                        .set_value("");
                    charging_summary_ctx
                        .label_energy_delivered_val
                        .set_value("");
                    charging_summary_ctx
                        .label_total_session_cost_val
                        .set_value("");
                    //
                    ctx.widget_evse.set_value(AssetPixmap::evse_ready());
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_init());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_msg_ready());
                }
                PlugState::Unknown => {
                    afb_log_msg!(Notice, None, "Display::PlugState::Unknown");
                }
            }
        }
        ChargingMsg::Iso(idata) => match idata {
            IsoState::Iso20 => {
                afb_log_msg!(Notice, None, "Display::IsoState::Iso20");
            }
            IsoState::Iso2 => {
                afb_log_msg!(Notice, None, "Display::IsoState::Iso2");
            }
            IsoState::Iso3 => {
                afb_log_msg!(Notice, None, "Display::IsoState::Iso3");
            }
            IsoState::Iec => {
                afb_log_msg!(Notice, None, "Display::IsoState::Iec");
            }
            IsoState::Unset => {
                afb_log_msg!(Notice, None, "Display::IsoState::Unset");
            }
        },

        ChargingMsg::Protocol(idata) => {
            match idata {
                ChargingProtocol::BasicCharge => {
                    ctx.charging_protocol.set(*idata);
                    afb_log_msg!(Notice, None, "Display::BasicCharge");
                    ctx.widget_evse.set_value(AssetPixmap::evse_simple());
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_charging());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_msg_charging_basic());
                    thread::sleep(Duration::from_secs(5));
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_charging_basiccharging());
                    ctx.widget_evse
                        .set_value(AssetPixmap::evse_state_msg_charging_evse());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_img_tesla());
                }
                ChargingProtocol::SmartCharge => {
                    afb_log_msg!(Notice, None, "Display::SmartCharge");
                    ctx.charging_protocol.set(*idata);
                    ctx.widget_evse.set_value(AssetPixmap::evse_simple());
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_charging());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_msg_charging_smart());
                    thread::sleep(Duration::from_secs(5));
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_charging_smartcharging());
                    ctx.widget_evse
                        .set_value(AssetPixmap::evse_state_msg_charging_evse());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_img_tesla());
                }
                ChargingProtocol::PlugAndCharge => {
                    afb_log_msg!(Notice, None, "Display::PlugAndCharge");
                    ctx.charging_protocol.set(*idata);
                    ctx.widget_evse.set_value(AssetPixmap::evse_simple());
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_charging());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_msg_charging_pnc());
                    thread::sleep(Duration::from_secs(5));
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_charging_plugncharge());
                    ctx.widget_evse
                        .set_value(AssetPixmap::evse_state_msg_charging_evse_genuine());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_img_tesla_genuine());
                }
                ChargingProtocol::Vehicle2Grid => {
                    // V2G Discharge
                    afb_log_msg!(Notice, None, "Display::Vehicle2Grid");
                    ctx.charging_protocol.set(*idata);
                    charging_timer_ctx.stop_animation.set(true);
                    ctx.widget_evse.set_value(AssetPixmap::evse_simple());
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_charging());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_msg_discharge_on());
                    thread::sleep(Duration::from_secs(5));
                    charging_timer_ctx.stop_animation.set(false);
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_charging_v2g());
                    ctx.widget_evse
                        .set_value(AssetPixmap::evse_state_msg_charging_evse());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_img_tesla());
                }
                ChargingProtocol::Grid2Vehicle => {
                    // V2G Charge
                    afb_log_msg!(Notice, None, "Display::Grid2Vehicle");
                    ctx.charging_protocol.set(*idata);
                    charging_timer_ctx.stop_animation.set(true);
                    ctx.widget_evse.set_value(AssetPixmap::evse_simple());
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_charging());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_msg_charging_v2g());
                    thread::sleep(Duration::from_secs(5));
                    charging_timer_ctx.stop_animation.set(false);
                    ctx.widget_progress
                        .set_value(AssetPixmap::charging_progress_charging_g2v());
                    ctx.widget_evse
                        .set_value(AssetPixmap::evse_state_msg_charging_evse());
                    ctx.widget_state_msg
                        .set_value(AssetPixmap::evse_state_img_tesla());
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn animate_flow(ctx: &MgrEvtChmgrCtrl) {
    let charging_timer_ctx = &ctx.charging_timer_ctx;
    let delay: u64 = 35;
    if !charging_timer_ctx.is_charging.get() {
        return;
    }
    // EVSE Sequence
    for state in [
        AssetPixmap::evse_state_msg_charging_evse_01(),
        AssetPixmap::evse_state_msg_charging_evse_02(),
        AssetPixmap::evse_state_msg_charging_evse_03(),
        AssetPixmap::evse_state_msg_charging_evse_04(),
        AssetPixmap::evse_state_msg_charging_evse_05(),
        AssetPixmap::evse_state_msg_charging_evse_06(),
        AssetPixmap::evse_state_msg_charging_evse_07(),
        AssetPixmap::evse_state_msg_charging_evse_08(),
    ] {
        ctx.widget_evse.set_value(state);
        thread::sleep(Duration::from_millis(delay));
        if !charging_timer_ctx.is_charging.get() {
            return;
        }
    }
    // Transition Sequence
    ctx.widget_evse
        .set_value(AssetPixmap::evse_state_msg_charging_evse_09());
    ctx.widget_state_msg
        .set_value(AssetPixmap::evse_state_img_tesla_01());
    thread::sleep(Duration::from_millis(delay));
    if !charging_timer_ctx.is_charging.get() {
        return;
    }

    ctx.widget_evse
        .set_value(AssetPixmap::evse_state_msg_charging_evse_10());
    ctx.widget_state_msg
        .set_value(AssetPixmap::evse_state_img_tesla_02());
    thread::sleep(Duration::from_millis(delay));
    if !charging_timer_ctx.is_charging.get() {
        return;
    }

    ctx.widget_evse
        .set_value(AssetPixmap::evse_state_msg_charging_evse());
    ctx.widget_state_msg
        .set_value(AssetPixmap::evse_state_img_tesla_03());
    thread::sleep(Duration::from_millis(delay));
    if !charging_timer_ctx.is_charging.get() {
        return;
    }
    // Vehicle Sequence
    for state in [
        AssetPixmap::evse_state_img_tesla_04(),
        AssetPixmap::evse_state_img_tesla_05(),
        AssetPixmap::evse_state_img_tesla_06(),
        AssetPixmap::evse_state_img_tesla_07(),
        AssetPixmap::evse_state_img_tesla_08(),
        AssetPixmap::evse_state_img_tesla_09(),
        AssetPixmap::evse_state_img_tesla_10(),
        AssetPixmap::evse_state_img_tesla(),
    ] {
        ctx.widget_state_msg.set_value(state);
        thread::sleep(Duration::from_millis(delay));
        if !charging_timer_ctx.is_charging.get() {
            return;
        }
    }
}

fn animate_pnc_flow(ctx: &MgrEvtChmgrCtrl) {
    let charging_timer_ctx = &ctx.charging_timer_ctx;
    let delay: u64 = 35;
    if !charging_timer_ctx.is_charging.get() {
        return;
    }
    // EVSE Sequence
    for state in [
        AssetPixmap::evse_state_msg_charging_evse_genuine_01(),
        AssetPixmap::evse_state_msg_charging_evse_genuine_02(),
        AssetPixmap::evse_state_msg_charging_evse_genuine_03(),
        AssetPixmap::evse_state_msg_charging_evse_genuine_04(),
        AssetPixmap::evse_state_msg_charging_evse_genuine_05(),
        AssetPixmap::evse_state_msg_charging_evse_genuine_06(),
        AssetPixmap::evse_state_msg_charging_evse_genuine_07(),
        AssetPixmap::evse_state_msg_charging_evse_genuine_08(),
    ] {
        ctx.widget_evse.set_value(state);
        thread::sleep(Duration::from_millis(delay));
        if !charging_timer_ctx.is_charging.get() {
            return;
        }
    }
    // Transition Sequence
    ctx.widget_evse
        .set_value(AssetPixmap::evse_state_msg_charging_evse_genuine_09());
    ctx.widget_state_msg
        .set_value(AssetPixmap::evse_state_img_tesla_genuine_01());
    thread::sleep(Duration::from_millis(delay));
    if !charging_timer_ctx.is_charging.get() {
        return;
    }

    ctx.widget_evse
        .set_value(AssetPixmap::evse_state_msg_charging_evse_genuine_10());
    ctx.widget_state_msg
        .set_value(AssetPixmap::evse_state_img_tesla_genuine_02());
    thread::sleep(Duration::from_millis(delay));
    if !charging_timer_ctx.is_charging.get() {
        return;
    }

    ctx.widget_evse
        .set_value(AssetPixmap::evse_state_msg_charging_evse_genuine());
    ctx.widget_state_msg
        .set_value(AssetPixmap::evse_state_img_tesla_genuine_03());
    thread::sleep(Duration::from_millis(delay));
    if !charging_timer_ctx.is_charging.get() {
        return;
    }
    // Vehicle Sequence
    for state in [
        AssetPixmap::evse_state_img_tesla_genuine_04(),
        AssetPixmap::evse_state_img_tesla_genuine_05(),
        AssetPixmap::evse_state_img_tesla_genuine_06(),
        AssetPixmap::evse_state_img_tesla_genuine_07(),
        AssetPixmap::evse_state_img_tesla_genuine_08(),
        AssetPixmap::evse_state_img_tesla_genuine_09(),
        AssetPixmap::evse_state_img_tesla_genuine_10(),
        AssetPixmap::evse_state_img_tesla_genuine(),
    ] {
        ctx.widget_state_msg.set_value(state);
        thread::sleep(Duration::from_millis(delay));
        if !charging_timer_ctx.is_charging.get() {
            return;
        }
    }
}

fn animate_discharge_flow(ctx: &MgrEvtChmgrCtrl) {
    let charging_timer_ctx = &ctx.charging_timer_ctx;
    let delay: u64 = 35;
    if !charging_timer_ctx.is_charging.get() {
        return;
    }
    // Vehicle Sequence
    for state in [
        AssetPixmap::evse_state_img_tesla_10(),
        AssetPixmap::evse_state_img_tesla_09(),
        AssetPixmap::evse_state_img_tesla_08(),
        AssetPixmap::evse_state_img_tesla_07(),
        AssetPixmap::evse_state_img_tesla_06(),
        AssetPixmap::evse_state_img_tesla_05(),
        AssetPixmap::evse_state_img_tesla_04(),
    ] {
        ctx.widget_state_msg.set_value(state);
        thread::sleep(Duration::from_millis(delay));
        if !charging_timer_ctx.is_charging.get() {
            return;
        }
    }
    // Transition Sequence
    ctx.widget_evse
        .set_value(AssetPixmap::evse_state_msg_charging_evse());
    ctx.widget_state_msg
        .set_value(AssetPixmap::evse_state_img_tesla_03());
    thread::sleep(Duration::from_millis(delay));
    if !charging_timer_ctx.is_charging.get() {
        return;
    }

    ctx.widget_evse
        .set_value(AssetPixmap::evse_state_msg_charging_evse_10());
    ctx.widget_state_msg
        .set_value(AssetPixmap::evse_state_img_tesla_02());
    thread::sleep(Duration::from_millis(delay));
    if !charging_timer_ctx.is_charging.get() {
        return;
    }

    ctx.widget_evse
        .set_value(AssetPixmap::evse_state_msg_charging_evse_09());
    ctx.widget_state_msg
        .set_value(AssetPixmap::evse_state_img_tesla_01());
    thread::sleep(Duration::from_millis(delay));
    if !charging_timer_ctx.is_charging.get() {
        return;
    }
    ctx.widget_state_msg
        .set_value(AssetPixmap::evse_state_img_tesla());
    // EVSE Sequence
    for state in [
        AssetPixmap::evse_state_msg_charging_evse_08(),
        AssetPixmap::evse_state_msg_charging_evse_07(),
        AssetPixmap::evse_state_msg_charging_evse_06(),
        AssetPixmap::evse_state_msg_charging_evse_05(),
        AssetPixmap::evse_state_msg_charging_evse_04(),
        AssetPixmap::evse_state_msg_charging_evse_03(),
        AssetPixmap::evse_state_msg_charging_evse_02(),
        AssetPixmap::evse_state_msg_charging_evse_01(),
        AssetPixmap::evse_state_msg_charging_evse(),
    ] {
        ctx.widget_evse.set_value(state);
        thread::sleep(Duration::from_millis(delay));
        if !charging_timer_ctx.is_charging.get() {
            return;
        }
    }
}

fn evt_auth_cb(
    event: &AfbEventMsg,
    args: &AfbRqtData,
    ctx_data: &AfbCtxData,
) -> Result<(), AfbError> {
    let ctx = ctx_data.get_ref::<MgrEvtAuthCrl>()?;
    afb_log_msg!(Notice, event, "-- evt_auth_cb event");
    let data = args.get::<&AuthMsg>(0)?;
    match data {
        AuthMsg::Done => {
            afb_log_msg!(Notice, None, "Display::AuthMsg::Done");
            ctx.widget_evse.set_value(AssetPixmap::evse_simple());
            ctx.widget_progress
                .set_value(AssetPixmap::charging_progress_auth_done());
            ctx.widget_state_msg
                .set_value(AssetPixmap::evse_state_msg_auth_done());
        }
        AuthMsg::Fail => {
            afb_log_msg!(Notice, None, "Display::AuthMsg::Fail");
            ctx.widget_evse.set_value(AssetPixmap::evse_simple());
            ctx.widget_progress
                .set_value(AssetPixmap::charging_progress_auth_fail());
            ctx.widget_state_msg
                .set_value(AssetPixmap::evse_state_msg_auth_fail());
        }
        AuthMsg::Pending => {
            afb_log_msg!(Notice, None, "Display::AuthMsg::Pending");
            ctx.widget_evse.set_value(AssetPixmap::evse_nfc_auth());
            ctx.widget_progress
                .set_value(AssetPixmap::charging_progress_nfc_auth());
            ctx.widget_state_msg
                .set_value(AssetPixmap::evse_state_msg_nfc());
        }
        AuthMsg::Idle => {
            afb_log_msg!(Notice, None, "Display::AuthMsg::Idle");
            // ctx.widget_evse.set_value(AssetPixmap::nfc_idle());
        }
    };
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
            return afb_error!(
                "conf-time-widget",
                "no widget uid: time  type:LvglLabel found in panel",
            )
        }
    };

    let date = match display.get_by_uid("date").downcast_ref::<LvglLabel>() {
        Some(widget) => widget,
        None => {
            return afb_error!(
                "conf-date-widget",
                "no widget uid: date  type:LvglLabel found in panel",
            )
        }
    };

    let label_elapsed_time = match display
        .get_by_uid("TimeElapsedVal")
        .downcast_ref::<LvglLabel>()
    {
        Some(widget) => widget,
        None => {
            return afb_error!(
                "conf-TimeElapsedVal-widget",
                "no widget uid: TimeElapsedVal  type:LvglLabel found in panel",
            )
        }
    };
    //------------------------------------------------------------------
    let engy_api = config.engy_api;
    let chmgr_api = config.chmgr_api;
    let auth_api = config.auth_api;

    let widget_evse = match display
        .get_by_uid("evse-status")
        .downcast_ref::<LvglPixmap>()
    {
        Some(widget) => widget,
        None => {
            return afb_error!(
                "verb-info-widget",
                "no widget uid:{} type:{} found in panel",
                "evse-status",
                "LvglPixmap"
            )
        }
    };

    let widget_state_msg = match display.get_by_uid("state-msg").downcast_ref::<LvglPixmap>() {
        Some(widget) => widget,
        None => {
            return afb_error!(
                "verb-info-widget",
                "no widget uid:{} type:{} found in panel",
                "state-msg",
                "LvglPixmap"
            )
        }
    };

    let widget_progress = match display
        .get_by_uid("Charge-progress")
        .downcast_ref::<LvglPixmap>()
    {
        Some(widget) => widget,
        None => {
            return afb_error!(
                "verb-info-widget",
                "no widget uid:{} type:{} found in panel",
                "Charge-progress",
                "LvglPixmap"
            )
        }
    };

    // Session Summary

    let label_summary_session = match display.get_by_uid("SummaryTxt").downcast_ref::<LvglLabel>() {
        Some(label) => label,
        None => {
            return afb_error!(
                "verb-info-label",
                "no label uid:{} type:{} found in panel",
                "SummaryTxt",
                "LvglLabel"
            )
        }
    };

    let label_charging_protocol = match display
        .get_by_uid("ProtocolTxt")
        .downcast_ref::<LvglLabel>()
    {
        Some(label) => label,
        None => {
            return afb_error!(
                "verb-info-label",
                "no label uid:{} type:{} found in panel",
                "ProtocolTxt",
                "LvglLabel"
            )
        }
    };

    let label_charging_duration = match display
        .get_by_uid("ChargingTimeTxt")
        .downcast_ref::<LvglLabel>()
    {
        Some(label) => label,
        None => {
            return afb_error!(
                "verb-info-label",
                "no label uid:{} type:{} found in panel",
                "ChargingTimeTxt",
                "LvglLabel"
            )
        }
    };

    let label_summary_energy_delivered = match display
        .get_by_uid("SessionEnergyDeliveredTxt")
        .downcast_ref::<LvglLabel>()
    {
        Some(label) => label,
        None => {
            return afb_error!(
                "verb-info-label",
                "no label uid:{} type:{} found in panel",
                "SessionEnergyDeliveredTxt",
                "LvglLabel"
            )
        }
    };

    let label_total_session_cost = match display
        .get_by_uid("TotalSessionCostTxt")
        .downcast_ref::<LvglLabel>()
    {
        Some(label) => label,
        None => {
            return afb_error!(
                "verb-info-label",
                "no label uid:{} type:{} found in panel",
                "TotalSessionCostTxt",
                "LvglLabel"
            )
        }
    };

    let label_charging_protocol_val = match display
        .get_by_uid("ProtocolVal")
        .downcast_ref::<LvglLabel>()
    {
        Some(label) => label,
        None => {
            return afb_error!(
                "verb-info-label",
                "no label uid:{} type:{} found in panel",
                "ProtocolVal",
                "LvglLabel"
            )
        }
    };

    let label_charging_duration_val = match display
        .get_by_uid("ChargingTimeVal")
        .downcast_ref::<LvglLabel>()
    {
        Some(label) => label,
        None => {
            return afb_error!(
                "verb-info-label",
                "no widget uid:{} type:{} found in panel",
                "ChargingTimeVal",
                "LvglLabel"
            )
        }
    };

    let label_energy_delivered_val = match display
        .get_by_uid("SessionEnergyDeliveredVal")
        .downcast_ref::<LvglLabel>()
    {
        Some(label) => label,
        None => {
            return afb_error!(
                "verb-info-label",
                "no label uid:{} type:{} found in panel",
                "SessionEnergyDeliveredVal",
                "LvglLabel"
            )
        }
    };

    let label_total_session_cost_val = match display
        .get_by_uid("TotalSessionCostVal")
        .downcast_ref::<LvglLabel>()
    {
        Some(label) => label,
        None => {
            return afb_error!(
                "verb-info-label",
                "no label uid:{} type:{} found in panel",
                "TotalSessionCostVal",
                "LvglLabel"
            )
        }
    };

    let label_energy_delivered = match display
        .get_by_uid("EnergyDeliveredVal")
        .downcast_ref::<LvglLabel>()
    {
        Some(label) => label,
        None => {
            return afb_error!(
                "verb-info-label",
                "no label uid:{} type:{} found in panel",
                "EnergyDeliveredVal",
                "LvglLabel"
            )
        }
    };

    let label_current_power = match display
        .get_by_uid("CurrentPowerVal")
        .downcast_ref::<LvglLabel>()
    {
        Some(label) => label,
        None => {
            return afb_error!(
                "verb-info-label",
                "no label uid:{} type:{} found in panel",
                "CurrentPowerVal",
                "LvglLabel"
            )
        }
    };

    let charging_timer_ctx = Arc::new(ChargingTimerCtx {
        elapsed_time: label_elapsed_time,
        start_charging_time: Cell::new(SystemTime::now()),
        stop_animation: Cell::new(false),
        is_charging: Cell::new(false),
        elapsed_time_val: RefCell::new(String::from("00:00")),
    });

    let charging_summary_ctx = Arc::new(ChargingSummaryCtx {
        label_summary_session,
        label_charging_protocol,
        label_charging_duration,
        label_summary_energy_delivered,
        label_total_session_cost,
        label_charging_protocol_val,
        label_charging_duration_val,
        label_energy_delivered_val,
        label_total_session_cost_val,
    });

    let energy_ctx = Arc::new(MgrEvtEngyCtrl {
        label_energy_delivered,
        label_current_power,
        energy_deliverd: RefCell::new(String::from("0.00")),
        total_cost: RefCell::new(String::from("00.00")),
    });

    let animation_timer_ctx = Arc::new(MgrEvtChmgrCtrl {
        widget_evse,
        widget_progress,
        widget_state_msg,
        label_elapsed_time,
        charging_timer_ctx: charging_timer_ctx.clone(),
        charging_summary_ctx: charging_summary_ctx.clone(),
        charging_protocol: Cell::new(ChargingProtocol::BasicCharge),
        energy_ctx: energy_ctx.clone(),
    });

    AfbTimer::new("charging-timer")
        .set_period(1000)
        .set_callback(charging_timer_cb)
        .set_context(charging_timer_ctx.clone())
        .start()?;

    AfbTimer::new("animation-timer")
        .set_period(1000)
        .set_callback(animation_timer_cb)
        .set_context(animation_timer_ctx.clone())
        .start()?;

    // Register the event handler
    let charger_handler = AfbEvtHandler::new("Charger_manager")
        .set_info("Charger Manager")
        .set_pattern(to_static_str(format!("{}/{}", config.chmgr_api, "*")))
        .set_callback(evt_chmgr_cb)
        .set_context(animation_timer_ctx.clone())
        .finalize()?;

    let auth_handler = AfbEvtHandler::new("Auth_manager")
        .set_info("Auth Manager")
        .set_pattern(to_static_str(format!("{}/{}", auth_api, "*")))
        .set_callback(evt_auth_cb)
        .set_context(MgrEvtAuthCrl {
            widget_evse,
            widget_progress,
            widget_state_msg,
        })
        .finalize()?;

    let energy_handler = AfbEvtHandler::new("Energy_manager")
        .set_info("Energy Manager")
        .set_pattern(to_static_str(format!("{}/{}", engy_api, "energy")))
        .set_callback(evt_nrj_cb)
        .set_context(energy_ctx.clone())
        .finalize()?;

    let power_handler = AfbEvtHandler::new("Power_manager")
        .set_info("Energy Manager")
        .set_pattern(to_static_str(format!("{}/{}", engy_api, "power")))
        .set_callback(evt_nrj_cb)
        .set_context(energy_ctx.clone())
        .finalize()?;

    api.add_evt_handler(energy_handler);
    api.add_evt_handler(power_handler);
    api.add_evt_handler(charger_handler);
    api.add_evt_handler(auth_handler);

    AfbTimer::new("clock-timer")
        .set_period(1000)
        .set_callback(timer_callback)
        .set_context(TimerCtx { time, date })
        .start()?;

    Ok(())
}
