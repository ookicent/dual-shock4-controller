/********************************************************
* Original work Copyright (C): 2020-2021 OOKI CENT,Ltd. All Rights Reserved.
* Modified work Copyright 2020 Thomas Gauthier-Caron
* FileName:  joystick.rs
* Author: http://www.ookicent.com
* Date: 2020-08-19  10:36
* Version: 0.1.2
* Description:
**********************************************************/

extern crate hidapi;
pub use crate::gamepad::GamePad;
use hidapi::{HidApi, HidDevice, HidError, HidResult};

#[derive(Copy, Clone, Debug)]
pub struct DeviceInfo {
    pub vid: u16,
    pub pid: u16,
}
impl DeviceInfo {}
pub struct Joystick {
    m_api: HidApi,
    m_gamepad: GamePad,
    m_device: HidDevice,
    m_color: Color,
    m_rumble: Rumble,
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub struct Rumble {
    left: u8,
    right: u8,
}

impl Joystick {
    pub fn new(rhs: DeviceInfo) -> Result<Joystick, HidError> {
        let api = hidapi::HidApi::new().unwrap();
        let gamepad = GamePad::new();

        let device = api.open(rhs.vid, rhs.pid)?;

        Ok(Joystick {
            m_api: api,
            m_gamepad: gamepad,
            m_device: device,
            m_color: Color { r: 0, g: 0, b: 0 },
            m_rumble: Rumble { left: 0, right: 0 },
        })
    }
    pub fn get_gamepad(&self) -> &GamePad {
        &self.m_gamepad
    }

    pub fn read(&self, buf: &mut [u8]) -> HidResult<usize> {
        return self.m_device.read(buf);
    }

    pub fn read_timeout(&self, buf: &mut [u8], timeout: i32) -> HidResult<usize> {
        self.m_device.read_timeout(buf, timeout)
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.m_color.r = r;
        self.m_color.g = g;
        self.m_color.b = b;
    }

    pub fn set_rumble(&mut self, left: u8, right: u8) {
        self.m_rumble.left = left;
        self.m_rumble.right = right;
    }

    pub fn update_led_and_rumble(&self) -> HidResult<usize> {
        self.m_device.write(&[
            5,
            255,
            4,
            0,
            self.m_rumble.left,
            self.m_rumble.right,
            self.m_color.r,
            self.m_color.b,
            self.m_color.g,
            0,
            0,
        ])
    }
}
