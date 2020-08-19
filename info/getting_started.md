<!--
 * @Descripttion: 
 * @version: 1.0.0
 * @Copyright: Copyright (C): http://www.ookicent.com
 * @Date: 2020-08-19 10:17:04
 * @LastEditors: OOKI CENT,Ltd
 * @LastEditTime: 2020-08-19 10:57:49
-->
# Getting Started

## Dependencies

Add this to your `Cargo.toml`:

```toml
dual-shock4-controller = "0.1.0"
```

## How To Use

```rust
    use dual_shock4_controller::joystick::{DeviceInfo,Joystick};
    
    let joystick = Joystick::new();
    let device_info = DeviceInfo{vid:0x054c,pid:0x05c4};//HID\VID_054C&PID_05C4\7&3869AC07&0&0000
    let device = joystick.connect(device_info).expect("can't find device!");//
    loop {
        let mut buf = [0u8;64];
        device.read_timeout(&mut buf[..], 1000).unwrap();
        let gamepad = joystick.get_gamepad().get_state(&buf);
        if gamepad.x_button.pressed {
            println!("× button is pressed");
            break;
        }
    }
```

