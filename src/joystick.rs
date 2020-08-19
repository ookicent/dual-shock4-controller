/********************************************************
* Copyright (C): 2020-2021 OOKI CENT,Ltd. All Rights Reserved. 
* FileName:  joystick.rs  
* Author: http://www.ookicent.com
* Date: 2020-08-19  10:36
* Version: 0.1.1
* Description: 
**********************************************************/

extern crate hidapi;
use hidapi::{
    HidApi,
    HidDevice,
    HidResult
};
pub use  crate::{
    gamepad::GamePad
};

#[derive(Copy, Clone, Debug)]
pub struct  DeviceInfo{
    pub vid:u16,
    pub pid:u16
}
impl  DeviceInfo{}
pub struct  Joystick{
    m_api:HidApi,
    m_gamepad:GamePad
}
impl Joystick{
    pub fn new()->Self{
        let api = hidapi::HidApi::new().unwrap();
        let gamepad = GamePad::new();
        Joystick{
            m_api:api,
            m_gamepad: gamepad
        }
    }
    pub fn get_gamepad(&self) ->&GamePad{
        &self.m_gamepad
    }
    pub fn connect(&self,rhs:DeviceInfo)->HidResult<HidDevice>{
        self.m_api.open(rhs.vid,rhs.pid)
    }
}