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

    pub fn draw_panel(&mut self) -> &mut Self {
    /*
        self.panel.push(
            LvglIcon::new("Icon-logo",LvglMkImgBin::tux_evsex40(), 500, 500)
                .set_info("Demo logo Icon")
                .finalize(),
        );
    */
    
        self.panel.push(
            LvglImage::new("tux-evse2", "/home/ronan/IoT/Valeo/lvgl/tux-evse-webapp/src/assets/images/tux_evsex150.png", 0, 0)
                .finalize(),
        );

//            .load_image_from_path()
/* 
    self.panel.push(
        LvglImgButton::new( "test imgBut", "label", 500, 500, LvglColor::palette(LvglPalette::INDIGO),1000, 2000 )
            .finalize(),
    );
*/

/*
        self.panel.push(
            LvglIcon::new("Icon-wifi", LvglPixmap::WIFI, 1000, 0)
                .set_info("Demo Wifi Icon")
                .finalize(),
        );

        self.panel.push(
            LvglIcon::new("Icon-Nfc", LvglPixmap::SD_CARD, 950, 0)
                .set_color(LvglColor::rvb(255, 0, 0))
                .finalize(),
        );

        self.panel
            .push(LvglIcon::new("Icon-Battery", LvglPixmap::BATTERY_2, 900, 0).finalize());

        self.panel.push(
            LvglLed::new("Led-Red", 850, 5)
                .set_info("red led")
                .set_color(LvglColor::palette(LvglPalette::RED))
                .set_size(10, 10)
                .set_on(true)
                .finalize(),
        );

        self.panel.push(
            LvglLed::new("Led-Green", 800, 5)
                .set_color(LvglColor::rvb(0, 255, 0))
                .set_info("green led")
                .set_brightness(255)
                .set_size(10, 10)
                .set_on(true)
                .finalize(),
        );
*/
        let line_x_ofs = 10 ;
        let line_top_y_ofs = 150 ;
        let line_mid_y_ofs = 250 ;
        let line_bot_y_ofs = 400 ;
        let line_x_end = 1024-2*line_x_ofs;
        let line_width = 5;
        let line_color = LvglPalette::LIGHT_BLUE;

        let points_line_top = [
            LvglPoint { x: line_x_ofs,       y: line_top_y_ofs },
            LvglPoint { x: line_x_end,       y: line_top_y_ofs },
        ];

        let points_line_mid = [
            LvglPoint { x: line_x_ofs,       y: line_mid_y_ofs },
            LvglPoint { x: line_x_end,       y: line_mid_y_ofs },
        ];

        let points_line_bot = [
            LvglPoint { x: line_x_ofs,       y: line_bot_y_ofs },
            LvglPoint { x: line_x_end,       y: line_bot_y_ofs },
        ];

        self.panel.push(
            LvglLine::new("line_top", 0, 0)
            .set_color(LvglColor::palette(line_color))
            .set_width(line_width)
            .set_rounded(true)
            .set_points(Box::new(points_line_top))
            .finalize(),
        );

        self.panel.push(
            LvglLine::new("line_mid", 0, 0)
            .set_color(LvglColor::palette(line_color))
            .set_width(line_width)
            .set_rounded(true)
            .set_points(Box::new(points_line_mid))
            .finalize(),
        );

        self.panel.push(
            LvglLine::new("line_bot", 0, 0)
            .set_color(LvglColor::palette(line_color))
            .set_width(line_width)
            .set_rounded(true)
            .set_points(Box::new(points_line_bot))
            .finalize(),
        );

        let label_date_x_ofs = 600;
        let label_time_x_ofs = 700;
        let label_date_y_ofs = line_top_y_ofs - 100 ;
        let label_date_height = 20;
        let label_time_height = label_date_height;
        
        self.panel.push(
            LvglLabel::new("date", "05/12/2023", label_date_x_ofs, label_date_y_ofs)
        .set_height(label_date_height)
        .finalize(),
        );

        self.panel.push(
            LvglLabel::new("time", "17:20:25", label_time_x_ofs, label_date_y_ofs)
        .set_height(label_time_height)
        .finalize(),
        );



        let switch_height = 20;
        let switch_title_height = 20;
        let switch_label_height = 15;
        let switch_x_ofs = 860;
        let switch_label_x_ofs = switch_x_ofs-70;
        let switch_sep = 5;
        let switch_iso_y_ofs = line_mid_y_ofs + 50;
        let switch_pnc_y_ofs = switch_iso_y_ofs+(switch_height+switch_sep)*1;
        let switch_v2g_y_ofs = switch_iso_y_ofs+(switch_height+switch_sep)*2;
        let switch_line_width = 6;
        let switch_line_color = LvglColor::palette(LvglPalette::CYAN);

        self.panel.push(
            LvglLabel::new("Label Switch", "Smart Charging", switch_x_ofs-50, switch_iso_y_ofs-switch_title_height-switch_sep)
        .set_height(switch_title_height)
        .finalize(),
        );

        self.panel.push(
            LvglLabel::new("Label Switch  iso", "ISO 15118", switch_label_x_ofs, switch_iso_y_ofs)
        .set_height(switch_title_height)
        .finalize(),
        );

        self.panel.push(
            LvglLabel::new("Label Switch pnc", "PnG", switch_label_x_ofs, switch_pnc_y_ofs)
        .set_height(switch_title_height)
        .finalize(),
        );

        self.panel.push(
            LvglLabel::new("Label Switch v2g", "V2G", switch_label_x_ofs, switch_v2g_y_ofs)
        .set_height(switch_title_height)
        .finalize(),
        );

        self.panel.push(
            LvglSwitch::new("Switch-iso", switch_x_ofs, switch_iso_y_ofs)
                .set_check(true)
                .set_height(switch_height)
                .finalize(),
        );

        self.panel.push(
            LvglSwitch::new("Switch-pnc", switch_x_ofs, switch_pnc_y_ofs)
                .set_check(true)
                .set_height(switch_height)
                .finalize(),
        );

        self.panel.push(
            LvglSwitch::new("Switch-v2g", switch_x_ofs, switch_v2g_y_ofs)
                .set_check(true)
                .set_height(switch_height)
                .finalize(),
        );

        let points_label = [
            LvglPoint { x: switch_label_x_ofs - (switch_height + 10),       y: switch_iso_y_ofs - (switch_title_height + switch_sep + 10) },
            LvglPoint { x: switch_label_x_ofs - (switch_height + 10),       y: switch_v2g_y_ofs + (switch_title_height + switch_sep + 10) },
            LvglPoint { x: switch_x_ofs       + (switch_height + 10 + 50), y: switch_v2g_y_ofs + (switch_title_height + switch_sep + 10)},
            LvglPoint { x: switch_x_ofs       + (switch_height + 10 + 50), y: switch_iso_y_ofs - (switch_title_height + switch_sep + 10) },
            LvglPoint { x: switch_label_x_ofs - (switch_height + 10),       y: switch_iso_y_ofs - (switch_title_height + switch_sep + 10)},
        ];
        self.panel.push(
            LvglLine::new("Line switch", 0, 0)
                .set_color(switch_line_color)
                .set_width(switch_line_width)
                .set_rounded(true)
                .set_points(Box::new(points_label))
                .finalize(),
        );

        let label_charge_info_x_ofs = 450;
        let label_charge_info_height = 30;
        let label_charge_info_y_ofs    = switch_iso_y_ofs - switch_title_height - switch_sep ;
        let label_charge_total_y_ofs    = label_charge_info_y_ofs + label_charge_info_height;
        let label_charge_duration_y_ofs = label_charge_info_y_ofs + 2*label_charge_info_height;

        self.panel.push(
            LvglLabel::new("Charge info", "Charge Information", label_charge_info_x_ofs, label_charge_info_y_ofs)
        .set_height(label_charge_info_height)
        .finalize(),
        );

        self.panel.push(
            LvglLabel::new("ChargeTotal", "Energy: 13477.1 Kwh Total", label_charge_info_x_ofs, label_charge_total_y_ofs)
        .set_height(label_charge_info_height)
        .finalize(),
        );

        self.panel.push(
            LvglLabel::new("ChargeDuration", "Duration: 01:02:46", label_charge_info_x_ofs, label_charge_duration_y_ofs)
        .set_height(label_charge_info_height)
        .finalize(),
        );

        let label_status_bat_x_ofs = 50;
        let label_status_bat_y_ofs = switch_iso_y_ofs - switch_title_height - switch_sep ;
        let label_status_bat_height = 20;

        self.panel.push(
            LvglLabel::new("Status Bat", "Status battery", label_status_bat_x_ofs, label_status_bat_y_ofs)
        .set_height(label_status_bat_height)
        .finalize(),
        );

        self.panel.push(
            LvglLabel::new("BatConso", "2760.4kw", label_status_bat_x_ofs, label_status_bat_y_ofs+100)
        .set_height(label_status_bat_height)
        .finalize(),
        );

        let label_zone_mess_x_ofs = 200;
        let label_zone_mess_y_ofs = line_bot_y_ofs + 10 ;
        let label_zone_mess_height = 800;

        self.panel.push(
            LvglTextArea::new("ZoneMessage", label_zone_mess_x_ofs, label_zone_mess_y_ofs)
                .set_info("Zone Message")
                .set_width(label_zone_mess_height)
                .set_text("No matter where you go there you are")
                .finalize(),
        );

        /*
        self.panel.push(
            LvglButton::new("Button-A", "Test-1", 100, 200)
                .set_info("Demo Button 1")
                .set_size(180, 100)
                .finalize(),
        );
*/
/* 
        self.panel.push(
            LvglButton::new("Button-B", "Test-2", 300, 200)
                .set_info("push button 1")
                .finalize(),
        );
*/

/*
        self.panel.push(
            LvglArc::new("Arc", 10, 270, 800, 150)
                .set_info("Arc widget")
                .finalize(),
        );

        self.panel.push(
            LvglBar::new("Bar-1", 10, 90, 700, 300)
                .set_info("variable bar")
                .set_size(10,250)
                .set_gradient(true, LvglColor::palette(LvglPalette::GREEN), LvglColor::palette(LvglPalette::YELLOW))
                .set_value(60)
                .finalize(),
        );

        self.panel.push(
            LvglBar::new("Bar-2", 10, 90, 400, 300)
                .set_info("variable bar")
                .set_size(250,10)
                .set_gradient(false, LvglColor::palette(LvglPalette::GREEN), LvglColor::palette(LvglPalette::YELLOW))
                .set_value(40)
                .finalize(),
        );
*/

/*
        self.panel.push(
            LvglMeter::new(
                "Meter",
                4,
                -10,
                LvglColor::palette(LvglPalette::INDIGO),
                800,
                350,
            )
            .set_size(200, 200)
            .set_tic(
                3,
                10,
                41,
                10,
                8,
                LvglColor::palette(LvglPalette::BLUE_GREY),
                LvglColor::palette(LvglPalette::GREY),
            )
            .set_zone(0, 20, 4, LvglColor::palette(LvglPalette::RED))
            .set_zone(80, 100, 4, LvglColor::palette(LvglPalette::GREEN))
            .set_border(4, LvglColor::palette(LvglPalette::LIGHT_BLUE))
            .set_background(LvglColor::palette(LvglPalette::PINK))
            .set_value(50)
            .finalize(),
        );
*/

/*
        self.panel.push(
            LvglTextArea::new("Text-Area", 0, 550)
                .set_info("Demo Text area Zone")
                .set_width(600)
                .set_text("display message zone")
                .finalize(),
        );
*/
        self
    }

    pub fn finalize(&mut self) {
        // sort widget by uid and add them to pannel pool
        self.panel.sort_by(|a, b| a.get_uid().cmp(&b.get_uid()));
        for widget in &self.panel {
            widget.set_callback(self.ctrlbox);
        }
        // start lvgl main loop thread
        self.handle.start_loop();
    }
}
