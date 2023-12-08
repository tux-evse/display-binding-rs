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
    pub handle: LvglHandle,
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
        self.panel.push(
            LvglLabel::new(self.handle.get_root_widget(), "Label-1", LvglMkFont::std_22(), 50, 400)
                .set_info("Demo Label widget")
                .set_value("This is a label widget")
                .set_title("Label widget", 100, 75, LvglMkFont::std_10())
                .set_size(300, 100)
                .set_color(LvglColor::rvb(0, 0, 0))
                .set_background(LvglColor::rvb(0xFF, 0xFF, 0xFF))
                .set_border(3, LvglColor::rvb(0, 0xff, 0))
                .finalize(),
        );

        self.panel.push(
            LvglPixButton::new(self.handle.get_root_widget(),"Pixmap-Button", 450, 7)
                .set_info("Demo Pixmap Button")
                .set_value(AssetPixmap::nfc_off())
                .set_background(LvglColor::BLUE_GREY())
                .set_title("Clickable", 12, 6, LvglMkFont::std_10())
                .set_border(3, LvglColor::PURPLE())
                .finalize(),
        );

        self.panel.push(
            LvglPixmap::new(self.handle.get_root_widget(),"Icon-Charge", LvglIcon::WIFI, 1000, 0)
                .set_info("Demo Wifi Icon")
                .finalize(),
        );

        self.panel
            .push(LvglPixmap::new(self.handle.get_root_widget(),"Icon-Battery", LvglIcon::BATTERY_2, 900, 0).finalize());

        self.panel.push(
            LvglLed::new(self.handle.get_root_widget(),"Led-Red", 850, 5)
                .set_info("red led")
                .set_color(LvglColor::RED())
                .set_size(10, 10)
                .set_on(true)
                .finalize(),
        );

        self.panel.push(
            LvglLed::new(self.handle.get_root_widget(),"Led-Green", 800, 5)
                .set_color(LvglColor::rvb(0, 255, 0))
                .set_info("green led")
                .set_brightness(255)
                .set_size(10, 10)
                .set_on(true)
                .finalize(),
        );

        self.panel.push(
            LvglSwitch::new(self.handle.get_root_widget(),"Switch-1", 670, 5)
                .set_lock(false)
                .set_height(20)
                .finalize(),
        );

        self.panel.push(
            LvglSwitch::new(self.handle.get_root_widget(),"Switch-2", 600, 5)
                .set_lock(true)
                .set_height(20)
                .finalize(),
        );

        self.panel.push(
            LvglQrcode::new(self.handle.get_root_widget(),
                "qr-code",
                LvglColor::LIGHT_BLUE(),
                LvglColor::DEEP_PURPLE(),
                150,
                450,
                370,
            )
            .set_value("https://github.com/tux-evse")
            .set_title("tux-evse@github", 10, 0, LvglMkFont::std_14())
            .finalize(),
        );

        let points = [
            LvglPoint { x: 5, y: 5 },
            LvglPoint { x: 70, y: 70 },
            LvglPoint { x: 120, y: 10 },
            LvglPoint { x: 180, y: 60 },
            LvglPoint { x: 240, y: 10 },
        ];
        self.panel.push(
            LvglLine::new(self.handle.get_root_widget(),"Line", 400, 100)
                .set_color(LvglColor::RED())
                .set_width(8)
                .set_rounded(true)
                .set_points(Box::new(points))
                .finalize(),
        );

        self.panel.push(
            LvglButton::new(self.handle.get_root_widget(),"Button-A", LvglMkFont::std_18(), 100, 200)
                .set_value("My Button-A")
                .set_info("Push Button 1")
                .set_size(180, 100)
                .finalize(),
        );

        self.panel.push(
            LvglButton::new(self.handle.get_root_widget(),"Button-B", LvglMkFont::std_14(), 300, 200)
                .set_info("Push button B")
                .set_value("My Button-B")
                .finalize(),
        );

        self.panel.push(
            LvglArc::new(self.handle.get_root_widget(),"Arc", 10, 270, 800, 150)
                .set_info("Arc widget")
                .finalize(),
        );

        self.panel.push(
            LvglBar::new(self.handle.get_root_widget(),"Bar-1", 10, 90, 700, 300)
                .set_info("variable bar")
                .set_size(10, 250)
                .set_gradient(true, LvglColor::GREEN(), LvglColor::YELLOW())
                .set_value(60)
                .finalize(),
        );

        self.panel.push(
            LvglBar::new(self.handle.get_root_widget(),"Bar-2", 10, 90, 400, 300)
                .set_info("variable bar")
                .set_size(250, 10)
                .set_gradient(false, LvglColor::GREEN(), LvglColor::YELLOW())
                .set_value(40)
                .finalize(),
        );

        self.panel.push(
            LvglMeter::new(self.handle.get_root_widget(),"Meter", 4, -10, LvglColor::INDIGO(), 800, 350)
                .set_size(200, 200)
                .set_tic(3, 10, 41, 10, 8, LvglColor::BLUE_GREY(), LvglColor::GREY())
                .set_zone(0, 20, 4, LvglColor::RED())
                .set_zone(80, 100, 4, LvglColor::GREEN())
                .set_border(4, LvglColor::LIGHT_BLUE())
                .set_background(LvglColor::PINK())
                .set_value(50)
                .finalize(),
        );

        self.panel.push(
            LvglTextArea::new(self.handle.get_root_widget(),"Text-Area", 0, 550)
                .set_info("Demo Text area Zone")
                .set_width(600)
                .set_value("display message zone")
                .finalize(),
        );

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
