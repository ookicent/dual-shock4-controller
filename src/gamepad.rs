/********************************************************
* Copyright (C): 2020-2021 OOKI CENT,Ltd. All Rights Reserved. 
* FileName:  gamepad.rs  
* Author: http://www.ookicent.com
* Date: 2020-08-19  10:36
* Version: 1.0.0
* Description: 
**********************************************************/


#[derive(Copy, Clone, Debug)]
pub struct  TouchPad{
    pub x:u32,
    pub y:u32,
    pub is_active:bool
}
#[derive(Copy, Clone, Debug)]
pub struct  StickInfo{
    pub x:u8,
    pub y:u8
}
#[derive(Copy, Clone, Debug)]
pub struct Stick{
    pub left_stick:StickInfo,
    pub right_stick:StickInfo,
}

#[derive(Copy, Clone, Debug)]
pub struct  Button{
    pub pressed:bool
}

#[derive(Copy, Clone, Debug)]
pub struct  GamePad{
    pub up_left_button:Button,
    pub up_right_button:Button,
    pub down_left_button:Button,
    pub down_right_button:Button,
    pub left_button:Button,
    pub right_button:Button,
    pub up_button:Button,
    pub down_button:Button,
    pub l1_button:Button,
    pub l2_button:Button,
    pub r1_button:Button,
    pub r2_button:Button,
    pub x_button:Button,
    pub o_button:Button,
    pub square_button:Button,
    pub triangle_button:Button,
    pub share_button:Button,
    pub option_button:Button,
    pub ps_button:Button,
    pub left_stick_button:Button,
    pub right_stick_button:Button,
    pub stick:Stick
}

impl GamePad{
    pub fn new()->Self{
        GamePad{
            up_left_button: Button{pressed:false},
            up_right_button: Button{pressed:false},
            down_left_button: Button{pressed:false},
            down_right_button: Button{pressed:false},
            left_button: Button{pressed:false},
            right_button: Button{pressed:false},
            up_button: Button{pressed:false},
            down_button: Button{pressed:false},
            l1_button:Button{pressed:false},
            l2_button:Button{pressed:false},
            r1_button:Button{pressed:false},
            r2_button:Button{pressed:false},
            x_button:Button{pressed:false},
            o_button:Button{pressed:false},
            square_button:Button{pressed:false},
            triangle_button:Button{pressed:false},
            share_button:Button{pressed:false},
            option_button:Button{pressed:false},
            ps_button:Button{pressed:false},
            left_stick_button: Button{pressed:false},
            right_stick_button: Button{pressed:false},
            stick: Stick {
                left_stick:StickInfo{x:0,y:0},
                right_stick: StickInfo{x:0,y:0},
            }
        }
    }
    pub fn get_state(&self,buf:&[u8])->Self{
        GamePad{
            up_left_button: Button{pressed:check_button_pressed((0x05, 0x07, 0xf), buf)},
            up_right_button: Button{pressed:check_button_pressed((0x05, 0x01, 0xf), buf)},
            down_left_button: Button{pressed:check_button_pressed((0x05, 0x05, 0xf), buf)},
            down_right_button: Button{pressed:check_button_pressed((0x05, 0x03, 0xf), buf)},
            left_button: Button{pressed:check_button_pressed((0x05, 0x06, 0xf), buf)},
            right_button: Button{pressed:check_button_pressed((0x05, 0x02, 0xf), buf)},
            up_button: Button{pressed:check_button_pressed((0x05, 0x00, 0xf), buf)},
            down_button: Button{pressed:check_button_pressed((0x05, 0x04, 0xf), buf)},
            l1_button:Button{pressed:check_button_pressed((0x06, 0x01, 0xff), buf)},
            l2_button:Button{pressed:check_button_pressed((0x06,0x04,0xff),buf)},
            r1_button:Button{pressed:check_button_pressed((0x06,0x02,0xff),buf)},
            r2_button:Button{pressed:check_button_pressed((0x06,0x08,0xff),buf)},
            x_button:Button{pressed:check_button_pressed((0x05,0x20,0xff),buf)},
            o_button:Button{pressed:check_button_pressed((0x05,0x40,0xff),buf)},
            square_button:Button{pressed:check_button_pressed((0x05,0x10,0xff),buf)},
            triangle_button:Button{pressed:check_button_pressed((0x05,0x80,0xff),buf)},
            share_button:Button{pressed:check_button_pressed((0x06,0x10,0xff),buf)},
            option_button:Button{pressed:check_button_pressed((0x06,0x20,0xff),buf)},
            ps_button:Button{pressed:check_button_pressed((0x07,0x01,0xff),buf)},
            left_stick_button: Button{pressed:check_button_pressed((0x06,0x40,0xff),buf)},
            right_stick_button: Button{pressed:check_button_pressed((0x06,0x80,0xff),buf)},
            stick:check_stick_rock(buf)
        }
    }
}

pub fn check_button_pressed(conf:(usize,u8,u8),buf:&[u8])->bool{
    let rev = buf[conf.0] & conf.2;

    if conf.0 == 0x05 && conf.1 == 0x00 {
        return  rev == 0x00
    }

    let result = rev & conf.1;
    result == conf.1
}

pub fn check_stick_rock(buf:&[u8])->Stick{

    let left_stick = StickInfo{
        x: buf[0x01],
        y: buf[0x02],
    };

    let right_stick = StickInfo{
        x: buf[0x03],
        y: buf[0x04],
    };

    Stick{
        left_stick,
        right_stick
    }

}
