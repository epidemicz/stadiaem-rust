#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use vigem::raw::{LPVOID, PVIGEM_CLIENT, PVIGEM_TARGET, UCHAR};
use vigem::notification::*;
use vigem::*;

mod StadiaController;

static mut VIBRATION: (u8, u8) = (0, 0);

pub fn main() {
    // Make a vigem object, which alloc immediantely
    let mut vigem = Vigem::new();
    // connect our client to a VigemBus
    vigem.connect().unwrap();
    // Make a new target which represent XBOX360 controller
    let mut target = Target::new(TargetType::Xbox360);
    // Get controller state - as target isnt connected state is "Initialized"
    dbg!(target.state());
    // Add target to VigemBUS
    vigem.target_add(&mut target).unwrap();
    // Now it's connected!
    dbg!(target.state());
    
    let mut stadia = StadiaController::StadiaController::new();
   
    // It's a bit harder. We register notification. Handle will be called every time controller get forcefeedbacked
    vigem
        .x360_register_notification::<u8>(&target, Some(handle), 0)
        .unwrap();

    let stadia2 = stadia.clone();

    std::thread::spawn(move || {
        loop {
            unsafe {
                stadia2.vibrate_tuple(VIBRATION);
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });

    loop {
        let mut report = stadia.get_report();               

        //println!("{:#?}", report);

        let mut xbox_report = XUSBReport::default();

        if report.Left > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::DpadLeft }
        if report.Up > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::DpadUp }
        if report.Right > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::DpadRight }
        if report.Down > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::DpadDown }

        if report.A > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::A }
        if report.B > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::B }
        if report.X > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::X }
        if report.Y > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::Y }
        if report.L1 > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::LeftShoulder }
        if report.L3 > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::LeftThumb }
        if report.R1 > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::RightShoulder }
        if report.R3 > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::RightThumb }
        if report.Select > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::Back }
        if report.Start > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::Start }
        if report.Stadia > 0 { xbox_report.w_buttons = xbox_report.w_buttons + XButton::Guide }

        xbox_report.s_thumb_lx = (128 + report.LX as i16) << 8;
        xbox_report.s_thumb_ly = (128 - report.LY as i16) << 8;

        xbox_report.s_thumb_rx = (128 + report.RX as i16) << 8;
        xbox_report.s_thumb_ry = (128 - report.RY as i16) << 8;

        xbox_report.b_left_trigger = report.L2;
        xbox_report.b_right_trigger = report.R2;

        target.update(&xbox_report).unwrap();

        //println!("{:#?}", report);

        //std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

extern "C" fn handle(
    client: PVIGEM_CLIENT,
    target: PVIGEM_TARGET,
    large_motor: UCHAR,
    small_motor: UCHAR,
    led_number: UCHAR,
    user_data: LPVOID,
) {
    // make a safe absraction over all arguments
    let notification: X360Notification<u8> = X360Notification::new(
        client,
        target,
        large_motor,
        small_motor,
        led_number,
        user_data,
    );

    unsafe {
        VIBRATION = (notification.large_motor, notification.small_motor);
    }
}
