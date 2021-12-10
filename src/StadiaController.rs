use hidapi::*;

#[derive(Debug)]
pub struct StadiaReport {
    DATA_ID: usize,
    DATA_DPAD: usize,
    DATA_BUTTONS_1: usize,
    DATA_BUTTONS_2: usize,
    DATA_LX: usize,
    DATA_LY: usize,
    DATA_RX: usize,
    DATA_RY: usize,
    DATA_L2: usize,
    DATA_R2: usize,
    pub A: u8,
    pub B: u8,
    pub X: u8,
    pub Y: u8,
    pub Up: u8,
    pub Down: u8,
    pub Left: u8,
    pub Right: u8,
    pub L1: u8,
    pub R1: u8,
    pub L3: u8,
    pub R3: u8,
    pub Assistant: u8,
    pub Screenshot: u8,
    pub Start: u8,
    pub Select: u8,
    pub Stadia: u8,
    pub LX: u8,
    pub LY: u8,
    pub RX: u8,
    pub RY: u8,
    pub L2: u8,
    pub R2: u8,
}

impl StadiaReport {
    pub fn new() -> StadiaReport {
        StadiaReport {
            DATA_ID: 0x00,
            DATA_DPAD: 0x01,
            DATA_BUTTONS_1: 0x02,
            DATA_BUTTONS_2: 0x03,
            DATA_LX: 0x04,
            DATA_LY: 0x05,
            DATA_RX: 0x06,
            DATA_RY: 0x07,
            DATA_L2: 0x08,
            DATA_R2: 0x09,
            A: 0,
            B: 0,
            X: 0,
            Y: 0,
            Up: 0,
            Down: 0,
            Left: 0,
            Right: 0,
            L1: 0,
            R1: 0,
            L3: 0,
            R3: 0,
            Assistant: 0,
            Screenshot: 0,
            Start: 0,
            Select: 0,
            Stadia: 0,
            LX: 0,
            LY: 0,
            RX: 0,
            RY: 0,
            L2: 0,
            R2: 0,
        }
    }
}


pub struct StadiaController {
    pub device: HidDevice,
}

impl Clone for StadiaController {
    fn clone(&self) -> Self {
        StadiaController {
            device: self.device.clone(),
        }
    }
}

impl StadiaController {
    pub fn new() -> StadiaController {
        let api = hidapi::HidApi::new().unwrap();
        let (VID, PID) = (0x18D1, 0x9400);
        let hid = api.open(VID, PID).unwrap();

        StadiaController {
            device: hid,
        }
    }

    pub fn vibrate_tuple(&self, tuple: (u8, u8)) {
        self.vibrate(tuple.0, tuple.1);
    }

    pub fn vibrate(&self, large: u8, small: u8) {    
        let buf = 
        [
            0x05, 
            large, 
            large, 
            small, 
            small
        ];
    
        //println!("Attempting to write to device...");
        self.device.write(&buf).unwrap();
    }

    pub fn get_report(&mut self) -> StadiaReport {
        let mut data: [u8; 11] = [0; 11];
        self.device.read(&mut data).unwrap();

        // parse report data
        let mut report = StadiaReport::new();

        let mut scratch = data[report.DATA_BUTTONS_2];

        report.L3 = scratch & 0x01;
        report.R1 = scratch & 0x02;
        report.L1 = scratch & 0x04;
        report.Y = scratch & 0x08;
        report.X = scratch & 0x10;
        report.B = scratch & 0x20;
        report.A = scratch & 0x40;

        scratch = data[report.DATA_BUTTONS_1];
        report.Screenshot = scratch & 0x01;
        report.Assistant = scratch & 0x02;
        report.Stadia = scratch & 0x10;
        report.Start = scratch & 0x20;
        report.Select = scratch & 0x40;        
        report.R3 = scratch & 0x80;

        report.L2 = data[report.DATA_L2];
        report.R2 = data[report.DATA_R2];
        report.LX = data[report.DATA_LX];
        report.LY = data[report.DATA_LY];
        report.RX = data[report.DATA_RX];
        report.RY = data[report.DATA_RY];

        match data[report.DATA_DPAD] {
            0 => {
                report.Up = 255;
                report.Right = 0;
                report.Down = 0;
                report.Left = 0;
            },
            1 => {
                report.Up = 255;
                report.Right = 255;
                report.Down = 0;
                report.Left = 0;
            },
            2 => {
                report.Up = 0;
                report.Right = 255;
                report.Down = 0;
                report.Left = 0;
            },
            3 => {
                report.Up = 0;
                report.Right = 255;
                report.Down = 255;
                report.Left = 0;
            },
            4 => {
                report.Up = 0;
                report.Right = 0;
                report.Down = 255;
                report.Left = 0;
            },
            5 => {
                report.Up = 0;
                report.Right = 0;
                report.Down = 255;
                report.Left = 255;
            },
            6 => {
                report.Up = 0;
                report.Right = 0;
                report.Down = 0;
                report.Left = 255;
            },
            7 => {
                report.Up = 255;
                report.Right = 0;
                report.Down = 0;
                report.Left = 255;
            },
            _ => {
                report.Up = 0;
                report.Right = 0;
                report.Down = 0;
                report.Left = 0;
            }
        };

        report
    }
}