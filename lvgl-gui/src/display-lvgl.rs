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

        // let pixmap_nfc_x_ofs = 1024 - 5 * 40;
        let pixmap_net_x_ofs = 1024 - 3 * 40;
        let pixmap_wifi_level_x_ofs = 1024 - 2 * 40;
        let pixmap_wifi_x_ofs = 1024 - 1 * 40;
        // let pixmap_lang_x_ofs = 1024 - 1 * 40;

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

        self
    }

    pub fn draw_panel_top(&mut self, root: &LvglWidget) -> &mut Self {

        let pix_evse_icon_x_ofs = 190;
        let pix_evse_icon_y_ofs = 24;
        
        let pix_state_msg_x_ofs = 390;
        let pix_state_msg_y_ofs = 24;

        let pix_summary_msg_x_ofs = pix_state_msg_x_ofs + 110;
        let pix_summary_msg_y_ofs = pix_state_msg_y_ofs + 10;

        let pix_protocol_msg_x_ofs = pix_state_msg_x_ofs + 10;
        let pix_protocol_msg_y_ofs = pix_state_msg_y_ofs + 80;

        let pix_protocol_val_x_ofs = pix_state_msg_x_ofs + 320;

        let label_height = 45;

        self.panel.push(
            LvglPixmap::new(
                root,
                "evse-status",
                AssetPixmap::evse_ready(),
                pix_evse_icon_x_ofs,
                pix_evse_icon_y_ofs,
            )
            .set_info("Pixmap lang")
            .finalize(),
        );

        self.panel.push(
            LvglPixmap::new(
                root,
                "state-msg",
                AssetPixmap::evse_state_msg_ready(),
                pix_state_msg_x_ofs,
                pix_state_msg_y_ofs,
            )
            .set_info("Pixmap lang")
            .finalize(),
        );

        // Session Summary Labels

        self.panel.push(
            LvglLabel::new(
                root,
                "SummaryTxt",
                LvglMkFont::std_40(),
                pix_summary_msg_x_ofs,
                pix_summary_msg_y_ofs,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(66,133,244))
            .set_value("")
            .finalize(),
        );

        // Label: Protocol used
        self.panel.push(
            LvglLabel::new(
                root,
                "ProtocolTxt",
                LvglMkFont::std_30(),
                pix_protocol_msg_x_ofs,
                pix_protocol_msg_y_ofs,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(89,89,89))
            .set_value("")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "ProtocolVal",
                LvglMkFont::std_30(),
                pix_protocol_val_x_ofs,
                pix_protocol_msg_y_ofs,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(66,133,244))
            .set_value("")
            .finalize(),
        );
        //
        // Label: Charging Time
        self.panel.push(
            LvglLabel::new(
                root,
                "ChargingTimeTxt",
                LvglMkFont::std_30(),
                pix_protocol_msg_x_ofs,
                pix_protocol_msg_y_ofs + 50,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(89,89,89))
            .set_value("")
            .finalize(),
        );
        self.panel.push(
            LvglLabel::new(
                root,
                "ChargingTimeVal",
                LvglMkFont::std_30(),
                pix_protocol_val_x_ofs,
                pix_protocol_msg_y_ofs + 50,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(66,133,244))
            .set_value("")
            .finalize(),
        );
        //

        // Label: Energy Delivered
        self.panel.push(
            LvglLabel::new(
                root,
                "SessionEnergyDeliveredTxt",
                LvglMkFont::std_30(),
                pix_protocol_msg_x_ofs,
                pix_protocol_msg_y_ofs + 100,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(89,89,89))
            .set_value("")
            .finalize(),
        );
        self.panel.push(
            LvglLabel::new(
                root,
                "SessionEnergyDeliveredVal",
                LvglMkFont::std_30(),
                pix_protocol_val_x_ofs,
                pix_protocol_msg_y_ofs + 100,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(66,133,244))
            .set_value("")
            .finalize(),
        );
        //
        // Label: Total Session cost
        self.panel.push(
            LvglLabel::new(
                root,
                "TotalSessionCostTxt",
                LvglMkFont::std_30(),
                pix_protocol_msg_x_ofs,
                pix_protocol_msg_y_ofs + 150,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(89,89,89))
            .set_value("")
            .finalize(),
        );
        self.panel.push(
            LvglLabel::new(
                root,
                "TotalSessionCostVal",
                LvglMkFont::std_30(),
                pix_protocol_val_x_ofs,
                pix_protocol_msg_y_ofs + 150,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(66,133,244))
            .set_value("")
            .finalize(),
        );
        //
        //

        self
    }


    pub fn draw_panel_progress(&mut self, root: &LvglWidget) -> &mut Self {

        let pix_progress_x_ofs = 31;
        let pix_progress_y_ofs = 12;

        self.panel.push(
            LvglPixmap::new(
                root,
                "Charge-progress",
                AssetPixmap::charging_progress_init(),
                pix_progress_x_ofs,
                pix_progress_y_ofs,
            )
            .set_info("Pixmap lang")
            .finalize(),
        );

        self
    }


    pub fn draw_panel_info(&mut self, root: &LvglWidget) -> &mut Self {
        
        let time_txt_x_ofs = 60;
        let time_val_x_ofs = time_txt_x_ofs + 55;
        
        let power_txt_x_ofs = time_txt_x_ofs + 330;
        let power_val_x_ofs = power_txt_x_ofs + 50;
        let power_unit_x_ofs = power_val_x_ofs + 100;

        let energy_txt_x_ofs = power_txt_x_ofs + 314;
        let energy_val_x_ofs = energy_txt_x_ofs + 60;
        let energy_unit_x_ofs = energy_val_x_ofs + 100;

        let label_height = 45;

        let time_txt_y_ofs = 20;
        let time_val_y_ofs = time_txt_y_ofs + 40;

        let power_txt_y_ofs = time_txt_y_ofs;
        let power_val_y_ofs = power_txt_y_ofs + 40;


        self.panel.push(
            LvglLabel::new(
                root,
                "TimeElapsedTxt",
                LvglMkFont::std_30(),
                time_txt_x_ofs,
                time_txt_y_ofs,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(89,89,89))
            .set_info("Elapsed Time")
            .set_value("Elapsed Time")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "TimeElapsedVal",
                LvglMkFont::std_30(),
                time_val_x_ofs,
                time_val_y_ofs,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(89,89,89))
            .set_value("00:00")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "CurrentPowerTxt",
                LvglMkFont::std_30(),
                power_txt_x_ofs,
                power_txt_y_ofs,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(89,89,89))
            .set_info("Current Power")
            .set_value("Current Power")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "CurrentPowerVal",
                LvglMkFont::std_30(),
                power_val_x_ofs,
                power_val_y_ofs,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(89,89,89))
            .set_value("0.00")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "CurrentPowerUnit",
                LvglMkFont::std_30(),
                power_unit_x_ofs,
                power_val_y_ofs,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(89,89,89))
            .set_value("kW")
            .finalize(),
        );
        
        self.panel.push(
            LvglLabel::new(
                root,
                "EnergyDeliveredTxt",
                LvglMkFont::std_30(),
                energy_txt_x_ofs,
                time_txt_y_ofs,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(89,89,89))
            .set_info("Energy Delivered")
            .set_value("Energy Delivered")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "EnergyDeliveredVal",
                LvglMkFont::std_30(),
                energy_val_x_ofs,
                time_val_y_ofs,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(89,89,89))
            .set_value("0.0")
            .finalize(),
        );

        self.panel.push(
            LvglLabel::new(
                root,
                "EnergyDeliveredUnit",
                LvglMkFont::std_30(),
                energy_unit_x_ofs,
                time_val_y_ofs,
            )
            .set_height(label_height)
            .set_color(LvglColor::rvb(89,89,89))
            .set_value("kWh")
            .finalize(),
        );

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
    pub fn draw_init_panel(&mut self) -> &mut Self {
        let init_area = LvglArea::new(self.get_root(), "Area Init", 0, 0)
        .set_size(1024, 600)
        .set_padding(0, 0, 0, 0)
        .set_border(0, LvglColor::rvb(0, 0xff, 0))
        .finalize();

        self.panel.push(
            LvglPixmap::new(
                init_area,
                "evse-init",
                AssetPixmap::evse_init(),
                40,
                30,
            )
            .finalize(),
        );

        self
    }

    pub fn draw_panel(&mut self) -> &mut Self {
        let area_menu_posy = 0;
        let area_menu_sizey = 60;

        let area_top_posy = area_menu_sizey;
        let area_top_sizey = 300;

        let area_mid_posy = area_top_posy + area_top_sizey;
        let area_mid_sizey = 120;

        let area_bot_posy = area_mid_posy + area_mid_sizey;
        let area_bot_sizey = 600 - area_mid_sizey - area_top_sizey - area_menu_sizey;

        let area_menu = LvglArea::new(self.get_root(), "Area Menu", 0, area_menu_posy)
            .set_size(1024, area_menu_sizey)
            .set_padding(0, 0, 0, 0)
            .set_border(0, LvglColor::rvb(0, 0xff, 0))
            .finalize();

        let area_top = LvglArea::new(self.get_root(), "Area Top", 0, area_top_posy)
            .set_size(1024, area_top_sizey)
            .set_padding(0, 0, 0, 0)
            .set_border(0, LvglColor::rvb(0, 0xff, 0))
            .finalize();

        let area_mid = LvglArea::new(self.get_root(), "Area Mid", 0, area_mid_posy)
            .set_size(1024, area_mid_sizey)
            .set_padding(0, 0, 0, 0)
            .set_border(0, LvglColor::rvb(0, 0xff, 0))
            .finalize();

        let area_bot = LvglArea::new(self.get_root(), "Area Bot", 0, area_bot_posy)
            .set_size(1024, area_bot_sizey)
            .set_padding(0, 0, 0, 0)
            .set_border(0, LvglColor::rvb(0, 0xff, 0))
            .finalize();

        self.draw_panel_menu(area_menu);
        self.draw_panel_top(area_top);
        self.draw_panel_progress(area_mid);
        self.draw_panel_info(area_bot);
        
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
