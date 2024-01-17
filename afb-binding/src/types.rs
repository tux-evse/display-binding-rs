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

use afbv4::prelude::*;
use serde::{Deserialize, Serialize};

AfbDataConverter!(error_state, ErrorState);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE", untagged)]
pub enum ErrorState {
    ErrE,
    ErrDf,
    ErrRelay,
    ErrRdc,
    ErrOverCurrent,
    ErrPermanent,
    ErrVentilation,
}

//#[serde(rename_all = "SCREAMING-KEBAB-CASE", tag = "action")]
AfbDataConverter!(iec_state, IecState);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum IecState {
    Bdf,
    Ef,
    Unset,
}
//#[serde(rename_all = "SCREAMING-KEBAB-CASE", tag = "action")]
AfbDataConverter!(power_request, PowerRequest2del);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PowerRequest2del {
    Start,
    Stop,
}

//#[serde(rename_all = "SCREAMING-KEBAB-CASE", tag = "action")]

AfbDataConverter!(plug_state, PlugState2del);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PlugState2del {
    PlugIn,
    Lock,
    Error,
    PlugOut,
    Unknown,
}

//#[serde(rename_all = "SCREAMING-KEBAB-CASE", tag = "action")]

AfbDataConverter!(iso15118_state, Iso15118State);
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Iso15118State {
    Iso20,
    Iso2,
    Iec,
    Unset,
}

//#[serde(rename_all = "kebab-case")]
AfbDataConverter!(vehicle_state, VehicleState);
#[derive(Serialize, Deserialize, Debug)]
pub struct VehicleState {
    pub plugged: PlugState2del,
    pub power_request: PowerRequest2del,
    pub power_imax: u32,
    pub iso15118: Iso15118State,
    pub iec_state: IecState,
}

//#[serde(rename_all = "kebab-case", tag = "action")]
AfbDataConverter!(charger_nrj, ChargerNrj);
#[derive(Serialize, Deserialize, Debug)]
pub struct ChargerNrj {
    pub energy: String,
    pub charge_power: String,
    pub duration: String,
}

//#[serde(rename_all = "kebab-case", tag = "action")]
AfbDataConverter!(charger_net, ChargerNet);
#[derive(Serialize, Deserialize, Debug)]
pub struct ChargerNet {
    pub wifi_status: bool,
    pub mobile_status: bool,
    pub ethernet_status: bool,
    pub nfc_status: bool,
}

pub fn types_register() -> Result<(),AfbError> {
    iec_state::register()?;
    plug_state::register()?;
    vehicle_state::register()?;
    error_state::register()?;
    charger_nrj::register()?;
    charger_net::register()?;

    Ok(())
}