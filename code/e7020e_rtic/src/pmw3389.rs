/// PWM3389 gaming mouse sensor driver
use embedded_hal::{
    blocking::{
        delay::DelayUs,
        spi::{Transfer, Write},
    },
    digital::v2::OutputPin,
};

use rtt_target::rprintln;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Register {
    ProductId = 0x00,
    RevisionId = 0x01,
    Motion = 0x02,
    DeltaXL = 0x03,
    DeltaXH = 0x04,
    DeltaYL = 0x05,
    DeltaYH = 0x06,
    SQUAL = 0x07,
    RawDataSum = 0x08,
    MaximumRawdata = 0x09,
    MinimumRawdata = 0x0A,
    ShutterLower = 0x0B,
    ShutterUpper = 0x0C,
    RippleControl = 0x0D,
    ResolutionL = 0x0E,
    ResolutionH = 0x0F,
    Config2 = 0x10,
    AngleTune = 0x11,
    FrameCapture = 0x12,
    SROMEnable = 0x13,
    RunDownshift = 0x14,
    Rest1RateLower = 0x15,
    Rest1RateUpper = 0x16,
    Rest1Downshift = 0x17,
    Rest2RateLower = 0x18,
    Rest2RateUpper = 0x19,
    Rest2Downshift = 0x1A,
    Rest3RateLower = 0x1B,
    Rest3RateUpper = 0x1C,
    Observation = 0x24,
    DataOutLower = 0x25,
    DataOutUpper = 0x26,
    RawDataDump = 0x29,
    SROMId = 0x2A,
    MinSQRun = 0x2B,
    RawDataThreshold = 0x2C,
    Control2 = 0x2D,
    Config5L = 0x2E,
    Config5H = 0x2F,
    PowerUpReset = 0x3A,
    Shutdown = 0x3B,
    Calibrate = 0x3D,
    InverseProductID = 0x3F,
    LiftCutoffTune3 = 0x41,
    AngleSnap = 0x42,
    LiftCutoffTune1 = 0x4A,
    MotionBurst = 0x50,
    LiftCutoffTune1Timeout = 0x58,
    LiftCutoffTune1MinLength = 0x5A,
    SROMLoadBurst = 0x62,
    LiftConfig = 0x63,
    RawDataBurst = 0x64,
    LiftCutoffTune2 = 0x65,
    LiftCutoffTune2Timeout = 0x71,
    LiftCutoffTune2MinLength = 0x72,
    PWMPeriodCnt = 0x73,
    PWMWidthCnt = 0x74,
}

#[derive(Debug)]
pub struct Status {
    pub dx: i16,
    pub dy: i16,

    pub squal: u8,
    pub motion: bool,
    pub surface: bool,
}

impl Register {
    fn addr(self) -> u8 {
        self as u8
    }
}

pub struct Pmw3389<SPI, CS, D> {
    spi: SPI,
    cs: CS,
    pub delay: D,
    burst: bool,
    cpi_increment : u16,
    dpi : u16
}

impl<SPI, CS, D, E> Pmw3389<SPI, CS, D>
where
    SPI: Transfer<u8, Error = E> + Write<u8, Error = E>,
    CS: OutputPin,
    D: DelayUs<u32>,
{
    fn com_begin(&mut self) {
        self.cs.set_low().ok();
    }

    fn com_end(&mut self) {
        self.cs.set_high().ok();
    }

    /// Creates a new driver from a SPI peripheral and a NCS pin
    pub fn new(spi: SPI, cs: CS, delay: D) -> Result<Self, E> {
        let mut pmw3389 = Pmw3389 {
            spi,
            cs,
            delay,
            burst: false,
            cpi_increment : 10,
            dpi : 400
        };

        rprintln!("pmw3389 - reset");

        // ensure SPI is reset for proper communication
        pmw3389.com_end();
        pmw3389.delay.delay_us(40);
        pmw3389.com_begin();
        pmw3389.delay.delay_us(40);
        pmw3389.com_end();
        pmw3389.delay.delay_us(40); // not sure if needed

        // read product id, should be 0x47
        let id = pmw3389.product_id()?;
        rprintln!("product_id 0x{:x}", id);
        assert_eq!(id, 0x47);

        // returns the current SROM id
        let srom_id = pmw3389.read_register(Register::SROMId)?;
        rprintln!("srom_id {}, 0x{:x}", srom_id, srom_id);

        rprintln!("reset");

        // force reset of PMW3389
        pmw3389.write_register(Register::PowerUpReset, 0x5a)?;

        // wait for reboot
        pmw3389.delay.delay_us(50_000);

        // check that device is ready to receive new firmware
        let srom_id = pmw3389.read_register(Register::SROMId)?;
        // assert_eq!(srom_id, 0x0);

        // read registers 0x02 to 0x06 (and discard the data)
        pmw3389.read_register(Register::Motion)?;
        pmw3389.read_register(Register::DeltaXL)?;
        pmw3389.read_register(Register::DeltaXH)?;
        pmw3389.read_register(Register::DeltaYL)?;
        pmw3389.read_register(Register::DeltaYH)?;

        pmw3389.upload_firmware()?;

        // wait for firmware to start, not sure how long?
        pmw3389.delay.delay_us(50_000); // 50ms

        rprintln!("Calibration start...");

        // 7. Write to register 0x3D with value 0x80.
        // 8. Read register 0x3D at 1ms interval until value 0xC0 is obtained or read up to 55ms. This register read interval must be
        //    carried out at 1ms interval with timing tolerance of +/- 1%.
        // 9. Write to register 0x3D with value 0x00.
        // 10. Write 0x20 to register 0x10
        pmw3389.write_register(Register::Calibrate, 0x80)?;
        for i in 0..55 {
            if pmw3389.read_register(Register::Calibrate)? == 0xC0 {
                rprintln!("Calibrated in {}ms", i);
                break;
            }
            pmw3389.delay.delay_us(1_000); // 1ms
        }

        pmw3389.write_register(Register::Calibrate, 0x00)?;

        rprintln!("Optical Chip Initialized");

        // 0x00 to Config2 register for wired mouse, or
        // 0x20 for wireless mouse design.
        pmw3389.write_register(Register::Config2, 0x00)?;

        Ok(pmw3389)
    }

    pub fn read_register(&mut self, reg: Register) -> Result<u8, E> {
        self.com_begin();

        self.burst = false;

        let mut buffer = [reg.addr() & 0x7f];
        self.spi.transfer(&mut buffer)?;

        // tSRAD
        self.delay.delay_us(35);

        let mut buffer = [0];
        self.spi.transfer(&mut buffer)?;

        // tSCLK-NCS for read operation is 120ns
        self.delay.delay_us(1);

        self.com_end();

        // tSRW/tSRR (=20us) minus tSCLK-NCS
        self.delay.delay_us(19);

        Ok(buffer[0])
    }

    pub fn write_register(&mut self, reg: Register, byte: u8) -> Result<(), E> {
        self.com_begin();

        self.burst = false;

        let mut buffer = [reg.addr() | 0x80];
        self.spi.transfer(&mut buffer)?;

        // send
        let mut buffer = [byte];
        self.spi.transfer(&mut buffer)?;

        // tSCLK-NCS for write operation
        self.delay.delay_us(20);

        self.com_end();
        // tSWW/tSWR (=120us) minus tSCLK-NCS.
        // Could be shortened, but is looks like a safe lower bound

        self.delay.delay_us(100);

        Ok(())
    }

    /// Reads the ProductId register; should return `0x47`
    pub fn product_id(&mut self) -> Result<u8, E> {
        self.read_register(Register::ProductId)
    }

    // Increments the cpi value
    pub fn increment_dpi(&mut self,direction :i16) {
        if direction == -1 && self.dpi > self.cpi_increment+50 {
            self.dpi -= self.cpi_increment;
        } else if direction == 1 && self.dpi < 4000 {
            self.dpi += self.cpi_increment;
        }
        self.store_cpi().ok();

    }
    pub fn set_dpi(&mut self,cpi : u16) 
    {
        if cpi >= 50{
            self.dpi = cpi;
            self.store_cpi().ok();
        }
    }
    // Set CPI
    pub fn store_cpi(&mut self) -> Result<(), E> {
        let cpi = self.dpi / 50;
        self.write_register(Register::ResolutionH, (cpi >> 8) as u8)?;
        self.write_register(Register::ResolutionL, cpi as u8)
    }


    // Set Min SQ Run
    pub fn set_min_sq_run(&mut self, treshold: u8) -> Result<(), E> {
        self.write_register(Register::MinSQRun, treshold)
    }

    /// Read status
    pub fn read_status(&mut self) -> Result<Status, E> {
        if !self.burst {
            rprintln!("initiate burst mode");
            self.write_register(Register::MotionBurst, 0x0)?;
            self.burst = true;
        }

        self.com_begin();

        self.spi.transfer(&mut [Register::MotionBurst.addr()])?;

        self.delay.delay_us(35); // waits for tSRAD

        // read burst buffer, read only the 7 first bytes
        let mut buf = [0u8; 7];
        self.spi.transfer(&mut buf)?;

        // tSCLK-NCS for read operation is 120ns
        self.delay.delay_us(120);

        self.com_end();

        //     BYTE[00] = Motion
        //            ==> 7 bit: MOT (1 when motion is detected)
        //            ==> 3 bit: 0 when chip is on surface / 1 when off surface
        //     BYTE[01] = Observation
        //     BYTE[02] = Delta_X_L = dx (LSB)
        //     BYTE[03] = Delta_X_H = dx (MSB)
        //     BYTE[04] = Delta_Y_L = dy (LSB)
        //     BYTE[05] = Delta_Y_H = dy (MSB)
        //     BYTE[06] = SQUAL     = Surface Quality register, max 0x80
        //                          - Number of features on the surface = SQUAL * 8
        //     BYTE[07] = Raw_Data_Sum   = It reports the upper byte of an 18‐bit counter which sums all 1296 raw data in the current frame;
        //                                * Avg value = Raw_Data_Sum * 1024 / 1296
        //     BYTE[08] = Maximum_Raw_Data  = Max raw data value in current frame, max=127
        //     BYTE[09] = Minimum_Raw_Data  = Min raw data value in current frame, max=127
        //     BYTE[10] = Shutter_Upper     = Shutter LSB
        //     BYTE[11] = Shutter_Lower     = Shutter MSB, Shutter = shutter is adjusted to keep the average raw data values within normal operating ranges

        let motion = buf[0] & (1 << 7) != 0; // 1 motion / 0 no motion
        let surface = buf[0] & (1 << 3) != 0; // 0 if on surface / 1 if off surface

        let dx: i16 = (((buf[3] as u16) << 8) | buf[2] as u16) as i16;
        let dy: i16 = (((buf[5] as u16) << 8) | buf[4] as u16) as i16;

        let squal = buf[6];

        Ok(Status {
            dx,
            dy,
            squal,
            motion,
            surface,
        })
    }

    // Upload the firmware
    pub fn upload_firmware(&mut self) -> Result<(), E> {
        // send the firmware to the chip, cf p.18 of the datasheet
        // Serial.println("Uploading firmware...");
        rprintln!("Uploading firmware...");

        //Write 0 to Rest_En bit of Config2 register to disable Rest mode.
        // adns_write_reg(Config2, 0x20);
        // is this correct?
        self.write_register(Register::Config2, 0x00)?;

        // write 0x1d in SROM_enable reg for initializing
        // adns_write_reg(SROM_Enable, 0x1d);
        self.write_register(Register::SROMEnable, 0x1d)?;

        // wait for more than one frame period
        // delay(10);
        // assume that the frame rate is as low as 100fps...
        // even if it should never be that low
        self.delay.delay_us(10 * 1_000);

        // write 0x18 to SROM_enable to start SROM download
        self.write_register(Register::SROMEnable, 0x18)?;

        rprintln!("Begin transfer...");

        // write the SROM file (=firmware data)
        self.com_begin();

        // write burst destination address
        self.spi
            .transfer(&mut [Register::SROMLoadBurst.addr() | 0x80])?;

        self.delay.delay_us(15);

        // write firmware byte by byte
        let firmware = crate::srom::FIRMWARE_C;
        for i in firmware {
            let mut buff = [i];
            self.spi.transfer(&mut buff)?;
            self.delay.delay_us(15); // 15us delay between transfers
        }

        // Per: added this, seems adequate
        self.delay.delay_us(105);

        self.com_end();

        // Read the SROM_ID register to verify the ID before any other register reads or writes.
        let srom_id = self.read_register(Register::SROMId)?;
        rprintln!("srom_id {}, 0x{:x}", srom_id, srom_id);
        assert_eq!(srom_id, firmware[1]);

        Ok(())
    }
}