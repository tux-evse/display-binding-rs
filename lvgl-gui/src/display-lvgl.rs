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
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::prelude::*;
use lvgl::prelude::*;
use std::any::Any;

pub struct DisplayHandle {
    handle: LvglHandle,
    panel: Vec<&'static LvglWidget>,
    ctrlbox: Option<*mut dyn LvglHandler>,
}

impl DisplayHandle {
    pub fn create(x_res: i16, y_res: i16, ratio: u32) -> Self {
        let handle = LvglHandle::new(x_res, y_res, ratio);

        let display = DisplayHandle {
            handle,
            panel: Vec::new(),
            ctrlbox: None,
        };
        display
    }

    pub fn set_theme(
        &mut self,
        primary: LvglColor,
        secondary: LvglColor,
        dark: bool,
        font: &LvglFont,
    ) -> &mut Self {
        self.handle.set_theme(primary, secondary, dark, font);
        self
    }

    pub fn set_callback(&mut self, ctrlbox: Box<dyn LvglHandler>) -> &mut Self {
        self.ctrlbox = Some(Box::leak(ctrlbox));
        self
    }

    pub fn get_panel<'a>(&'a self) -> &'a Vec<&'static LvglWidget> {
        &self.panel
    }

    pub fn get_root(&self) -> &'static LvglWidget {
        self.handle.get_root_widget()
    }

    pub fn get_by_uid(&self, uid: &str) -> &'static dyn Any {
        let widget = match self
            .panel
            .binary_search_by(|widget| widget.get_uid().cmp(uid))
        {
            Ok(index) => self.panel[index].as_any(),
            Err(_) => &0, // return a dummy value
        };
        widget
    }

    pub fn draw_panel_menu(&mut self, root: &LvglWidget) -> &mut Self {
        let pixmap_logo_x_ofs = 110;
        let pixmap_logo_y_ofs = 20;

        let pixmap_logovaleo_x_ofs = 20;
        let pixmap_logovaleo_y_ofs = 15;

        //-----------------------------------------
        let pixmap_date_time_ico_y_ofs = 20;
        let label_time_height = 20;

        let pixmap_date_x_ofs = 380;
        let label_date_x_ofs = 415;

        let pixmap_time_x_ofs = 575;
        let label_time_x_ofs = 600;

        let label_date_height = 20;

        //-----------------------------------------
        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-iotbzh",
                AssetPixmap::logo_iot_bzh_flat(),
                pixmap_logo_x_ofs,
                pixmap_logo_y_ofs,
            )
            .set_info("Pixmap iotbzh")
            .finalize(),
        );

        //-----------------------------------------
        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-valeo",
                AssetPixmap::logo_valeo(),
                pixmap_logovaleo_x_ofs,
                pixmap_logovaleo_y_ofs,
            )
            .set_info("Pixmap valeo")
            .finalize(),
        );
//-----------------------------------------

        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-date",
                AssetPixmap::calendar3(),
                pixmap_date_x_ofs,
                pixmap_date_time_ico_y_ofs+2,
            )
            .set_info("Pixmap date")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "date",
                LvglMkFont::std_18(),
                label_date_x_ofs,
                pixmap_date_time_ico_y_ofs,
            )
            .set_height(label_date_height)
            .set_value("05/12/2023")
            .finalize(),
        );

        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-time",
                AssetPixmap::clock(),
                pixmap_time_x_ofs,
                pixmap_date_time_ico_y_ofs+2,
            )
            .set_info("Pixmap time")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "time",
                LvglMkFont::std_18(),
                label_time_x_ofs,
                pixmap_date_time_ico_y_ofs,
            )
            .set_height(label_time_height)
            .set_value("17:20:25")
            .finalize(),
        );
        //-----------------------------------------

        let pixmap_ico_y_ofs = 15;

        let pixmap_nfc_x_ofs = 1024 - 5 * 40;
        let pixmap_net_x_ofs = 1024 - 4 * 40;
        let pixmap_wifi_level_x_ofs = 1024 - 3 * 40;
        let pixmap_wifi_x_ofs = 1024 - 2 * 40;
        let pixmap_lang_x_ofs = 1024 - 1 * 40;

        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-nfc",
                AssetPixmap::nfc_off(),
                pixmap_nfc_x_ofs,
                pixmap_ico_y_ofs,
            )
            .set_info("Pixmap nfc")
            .finalize(),
        );
        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-net",
                AssetPixmap::ethernet_on(),
                pixmap_net_x_ofs,
                pixmap_ico_y_ofs,
            )
            .set_info("Pixmap net")
            .finalize(),
        );
        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-wifi_level",
                AssetPixmap::reception_on(),
                pixmap_wifi_level_x_ofs,
                pixmap_ico_y_ofs,
            )
            .set_info("Pixmap wifi_level")
            .finalize(),
        );
        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-wifi",
                AssetPixmap::wifi_on(),
                pixmap_wifi_x_ofs,
                pixmap_ico_y_ofs,
            )
            .set_info("Pixmap wifi")
            .finalize(),
        );
        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-lang",
                AssetPixmap::translate(),
                pixmap_lang_x_ofs,
                pixmap_ico_y_ofs,
            )
            .set_info("Pixmap lang")
            .finalize(),
        );

        self
    }

    pub fn draw_panel_top(&mut self, root: &LvglWidget) -> &mut Self {
        let pix_connect_status_x_ofs = 50;
        let pix_connect_status_y_ofs = 30;

        let pix_auth_status_x_ofs = 750;
        let pix_charge_status_x_ofs = 400;

        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-connect-status",
                AssetPixmap::plug_disconnected(),
                pix_connect_status_x_ofs,
                pix_connect_status_y_ofs,
            )
            .set_info("Pixmap lang")
            .finalize(),
        );

        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-charge-status",
                AssetPixmap::station_available(),
                pix_charge_status_x_ofs,
                pix_connect_status_y_ofs,
            )
            .set_info("Pixmap lang")
            .finalize(),
        );


        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-auth-status",
                AssetPixmap::nfc_idle(),
                pix_auth_status_x_ofs,
                pix_connect_status_y_ofs,
            )
            .set_info("Pixmap lang")
            .finalize(),
        );


        // self.panel.push(
        //     LvglPixButton::new(
        //         root,
        //         "Pixmap-start",
        //         pix_start_x_ofs,
        //         pix_connect_status_y_ofs,
        //     )
        //     .set_info("Pixmap lang")
        //     .set_value(AssetPixmap::btn_start())
        //     .set_disable(true)
        //     .finalize(),
        // );

        self
    }

    pub fn draw_panel_status_bat(&mut self, root: &LvglWidget) -> &mut Self {
        let label_status_bat_x_ofs = 20;
        let label_status_bat_y_ofs = 5;
        let label_status_bat_height = 30;

        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-lang",
                AssetPixmap::battery_charge_on(),
                label_status_bat_x_ofs + 20,
                label_status_bat_y_ofs,
            )
            .set_info("Pixmap lang")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "BatConso",
                LvglMkFont::std_22(),
                label_status_bat_x_ofs + 45,
                label_status_bat_y_ofs + 110,
            )
            .set_height(label_status_bat_height)
            .set_value("0.0")
            .finalize(),
        );
        LvglLabel::new(
            root,
            "BatConsoKw",
            LvglMkFont::std_22(),
            label_status_bat_x_ofs + 50 + 100,
            label_status_bat_y_ofs + 110,
        )
        .set_height(label_status_bat_height)
        .set_value("W") // the value is in Watt
        .finalize();

        self
    }

    pub fn draw_panel_info_charging(&mut self, root: &LvglWidget) -> &mut Self {
        //let pixmap_logo_x_ofs = 5;

        let label_txt_x_ofs = 10;
        let label_val_x_ofs = label_txt_x_ofs + 120;
        let label_unit_x_ofs = label_val_x_ofs + 90;


        let label_height = 45;

        let label_volts_y_ofs = 15;
        let label_amps_y_ofs = label_volts_y_ofs + label_height;
        let label_energy_y_ofs = label_volts_y_ofs + 2 * label_height;
        //let label_power_y_ofs = label_volts_y_ofs + 3 * label_height;
        //let label_adps_y_ofs = label_volts_y_ofs + 3 * label_height;

        self.panel.push(
            LvglLabel::new(
                root,
                "ChargeVoltsTxt",
                LvglMkFont::std_22(),
                label_txt_x_ofs,
                label_volts_y_ofs,
            )
            .set_height(label_height)
            .set_info("Voltage")
            .set_value("Voltage")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "ChargeVoltsVal",
                LvglMkFont::std_22(),
                label_val_x_ofs,
                label_volts_y_ofs,
            )
            .set_height(label_height)
            .set_value("0.0")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "ChargeVoltsUnit",
                LvglMkFont::std_22(),
                label_unit_x_ofs,
                label_volts_y_ofs,
            )
            .set_height(label_height)
            .set_value("V")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "ChargeImpsTxt",
                LvglMkFont::std_22(),
                label_txt_x_ofs,
                label_amps_y_ofs,
            )
            .set_height(label_height)
            .set_value("Current")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "ChargeImpsVal",
                LvglMkFont::std_22(),
                label_val_x_ofs,
                label_amps_y_ofs,
            )
            .set_height(label_height)
            .set_value("0.0")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "ChargeImpsUnit",
                LvglMkFont::std_22(),
                label_unit_x_ofs,
                label_amps_y_ofs,
            )
            .set_height(label_height)
            .set_value("A")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "ChargeEnergyTxt",
                LvglMkFont::std_22(),
                label_txt_x_ofs,
                label_energy_y_ofs,
            )
            .set_height(label_height)
            .set_value("Energy")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "ChargeEnergysVal",
                LvglMkFont::std_22(),
                label_val_x_ofs,
                label_energy_y_ofs,
            )
            .set_height(label_height)
            .set_value("0.1")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "ChargeEnergysUnit",
                LvglMkFont::std_22(),
                label_unit_x_ofs,
                label_energy_y_ofs,
            )
            .set_height(label_height)
            .set_value("kW.h")
            .finalize(),
        );

        self
    }

    pub fn draw_panel_smart_charging(&mut self, root: &LvglWidget) -> &mut Self {
        let switch_height = 20;
        let switch_title_height = 20;

        let switch_label_x_ofs = 15;
        let switch_x_ofs: i16 = switch_label_x_ofs + 160;
        let switch_sep = 20;

        let switch_main_label_x_ofs = switch_x_ofs - 50;
        let switch_main_label_y_ofs = 5;
        let switch_iec_y_ofs = switch_main_label_y_ofs + 35;
        let switch_pnc_y_ofs = switch_iec_y_ofs + (switch_height + switch_sep) * 1;
        let switch_iso_y_ofs = switch_iec_y_ofs + (switch_height + switch_sep) * 2;

        self.panel.push(
            LvglLabel::new(
                root,
                "Label Switch",
                LvglMkFont::std_18(),
                switch_main_label_x_ofs-70,
                switch_main_label_y_ofs,
            )
            .set_height(switch_title_height)
            .set_value("Smart Charging")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "IEC",
                LvglMkFont::std_22(),
                switch_label_x_ofs,
                switch_iec_y_ofs,
            )
            .set_height(switch_title_height)
            .set_value("IEC 61851")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "PnC",
                LvglMkFont::std_22(),
                switch_label_x_ofs,
                switch_pnc_y_ofs,
            )
            .set_height(switch_title_height)
            .set_value("PlugnC")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "Label Switch  iso",
                LvglMkFont::std_22(),
                switch_label_x_ofs,
                switch_iso_y_ofs,
            )
            .set_height(switch_title_height)
            .set_value("ISO 15118")
            .finalize(),
        );

        self.panel.push(
            LvglSwitch::new(root, "Switch-iec", switch_x_ofs, switch_iec_y_ofs)
                .set_disable(true)
                .set_height(switch_height)
                .set_value(false)
                .finalize(),
        );

        self.panel.push(
            LvglSwitch::new(root, "Switch-pnc", switch_x_ofs, switch_pnc_y_ofs)
                .set_disable(true)
                .set_height(switch_height)
                .set_value(false)
                .finalize(),
        );

        self.panel.push(
            LvglSwitch::new(root, "Switch-iso", switch_x_ofs, switch_iso_y_ofs)
                .set_disable(true)
                .set_height(switch_height)
                .set_value(false)
                .finalize(),
        );

        self
    }

    pub fn draw_panel_mid(&mut self, root: &LvglWidget) -> &mut Self {

        let area_status_bat_width = 220;
        let area_status_bat_height = 155;
        let area_status_bat_sizex = 30;
        let area_status_bat_sizey = 15;

        let area_smart_info_width = 300;
        let area_smart_info_height = 155;
        let area_smart_info_sizex = 350;
        let area_smart_info_sizey = area_status_bat_sizey;

        let area_smart_charging_width = 250;
        let area_smart_charging_height = 155;
        let area_smart_charging_sizex = 1024 - area_smart_charging_width - 50;
        let area_smart_charging_sizey = area_smart_info_sizey;

        let area_status_bat = LvglArea::new(
            root,
            "Area Status Bat",
            area_status_bat_sizex,
            area_status_bat_sizey,
        )
        .set_size(area_status_bat_width, area_smart_info_height)
        .set_padding(0, 0, 0, 0)
        .finalize();

        let area_info_charging = LvglArea::new(
            root,
            "Area info charging",
            area_smart_info_sizex,
            area_smart_info_sizey,
        )
        .set_size(area_smart_info_width, area_status_bat_height)
        .set_padding(0, 0, 0, 0)
        .finalize();

        let area_smart_charging = LvglArea::new(
            root,
            "Area smart charging",
            area_smart_charging_sizex,
            area_smart_charging_sizey,
        )
        .set_size(area_smart_charging_width, area_smart_charging_height)
        .set_padding(0, 0, 0, 0)
        .finalize();

        self.draw_panel_status_bat(area_status_bat);
        self.draw_panel_info_charging(area_info_charging);
        self.draw_panel_smart_charging(area_smart_charging);

        self
    }

    pub fn draw_panel_bot(&mut self, root: &LvglWidget) -> &mut Self {
        let bare_code_size = 130;

        let label_zone_mess_x_ofs = bare_code_size + 40;
        let label_zone_mess_y_ofs = label_zone_mess_x_ofs / 4;

        let pixmap_logo_x_ofs = 1024 - 170;
        let pixmap_logo_y_ofs = 0;


        let label_zone_mess_height = 1024 - label_zone_mess_x_ofs - 10 - 200;

        self.panel.push(
            LvglQrcode::new(
                root,
                "qr-code",
                LvglColor::rvb(255, 255, 255),
                LvglColor::rvb(0, 0, 0),
                bare_code_size,
                5,
                5,
            )
            .set_value("WIFI:T:WPA;S:tuxevse_hotspot;P:valeocharger;")
            .finalize(),
        );

        self.panel.push(
            LvglPixmap::new(
                root,
                "Pixmap-logo",
                AssetPixmap::tux_evsex150(),
                pixmap_logo_x_ofs,
                pixmap_logo_y_ofs,
            )
            .set_info("Pixmap nfc")
            .finalize(),
        );


        self.panel.push(
            LvglTextArea::new(
                root,
                "ZoneMessage",
                label_zone_mess_x_ofs,
                label_zone_mess_y_ofs,
            )
            .set_info("Zone Message")
            .set_width(label_zone_mess_height)
            .set_disable(true)
            .insert_text("A new text updated with OTA")
            .finalize(),
        );

        self
    }

    pub fn draw_panel(&mut self) -> &mut Self {
        let area_menu_posy = 0;
        let area_menu_sizey = 60;

        let area_top_posy = area_menu_sizey;
        let area_top_sizey = 210;

        let area_mid_posy = area_top_posy + area_top_sizey;
        let area_mid_sizey = 190;

        let area_bot_posy = area_mid_posy + area_mid_sizey;
        let area_bot_sizey = 600 - area_mid_sizey - area_top_sizey - area_menu_sizey;

        let area_menu = LvglArea::new(self.get_root(), "Area Menu", 0, area_menu_posy)
            .set_size(1024, area_menu_sizey)
            .set_padding(0, 0, 0, 0)
            .finalize();

        let area_top = LvglArea::new(self.get_root(), "Area Top", 0, area_top_posy)
            .set_size(1024, area_top_sizey)
            .set_padding(0, 0, 0, 0)
            .finalize();

        let area_mid = LvglArea::new(self.get_root(), "Area Mid", 0, area_mid_posy)
            .set_size(1024, area_mid_sizey)
            .set_padding(0, 0, 0, 0)
            .finalize();

        let area_bot = LvglArea::new(self.get_root(), "Area Bot", 0, area_bot_posy)
            .set_size(1024, area_bot_sizey)
            .set_padding(0, 0, 0, 0)
            .finalize();

        self.draw_panel_menu(area_menu);
        self.draw_panel_top(area_top);
        self.draw_panel_mid(area_mid);
        self.draw_panel_bot(area_bot);
        
        self
    }

    pub fn finalize(&mut self) {
        // sort widget by uid and add them to pannel pool
        self.panel.sort_by(|a, b| a.get_uid().cmp(&b.get_uid()));
        for widget in &self.panel {
            match self.ctrlbox {
                Some(callback) => widget.set_callback(callback),
                None => {}
            }
        }
        // start lvgl main loop thread
        self.handle.start_loop();
    }
}
