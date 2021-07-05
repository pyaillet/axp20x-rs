#![no_std]

use embedded_hal::blocking::{
    delay::DelayMs,
    i2c::{Read, Write, WriteRead},
};

const AXP202_CHIP_ID: u8 = 0x41;

const AXP202_SLAVE_ADDRESS: u8 = 0x35;

const AXP202_MODE_CHGSTATUS: u8 = 0x01;
const AXP202_IC_TYPE: u8 = 0x03;
const AXP202_LDO234_DC23_CTL: u8 = 0x12;
const AXP202_BATT_PERCENTAGE: u8 = 0xb9;

const AXP202_DCDC3: u8 = 1;
const AXP202_LDO2: u8 = 2;

macro_rules! bit_mask {
    ($x:expr) => {
        (1 << $x)
    };
}

macro_rules! is_open {
    ($reg:expr, $channel:expr) => {
        $reg & bit_mask!($channel) != 0
    };
}

#[derive(Debug)]
pub enum State {
    ON,
    OFF,
}

pub struct AXP20X<I2C: Read + Write + WriteRead> {
    i2c: I2C,
    chip_id: u8,
    init: bool,
}

#[derive(Debug)]
pub enum AXPError {
    WriteError,
    ReadError,
    InvalidState,
    UnsupportedOperation,
}

impl<I2C: Read + Write + WriteRead> AXP20X<I2C> {
    pub fn new(i2c: I2C) -> Self {
        AXP20X {
            i2c,
            chip_id: 0,
            init: false,
        }
    }

    pub fn init(&mut self, delay: &mut impl DelayMs<u32>) -> Result<(), AXPError> {
        let mut buf: [u8; 1] = [0];
        self.i2c
            .write_read(AXP202_SLAVE_ADDRESS, &[AXP202_IC_TYPE], &mut buf)
            .map_err(|_e| AXPError::ReadError)?;
        self.chip_id = buf[0];
        self.i2c
            .write_read(AXP202_SLAVE_ADDRESS, &[AXP202_LDO234_DC23_CTL], &mut buf)
            .map_err(|_e| AXPError::ReadError)?;
        self.set_power_output(AXP202_LDO2, State::ON, delay)?;
        self.init = true;
        Ok(())
    }

    fn write(&mut self, address: u8, reg: u8, cmd: u8) -> Result<(), AXPError> {
        self.i2c
            .write(address, &[reg, cmd])
            .map_err(|_e| AXPError::WriteError)
    }

    pub fn set_power_output(
        &mut self,
        channel: u8,
        state: State,
        delay: &mut impl DelayMs<u32>,
    ) -> Result<(), AXPError> {
        // Before setting, the output cannot be all turned off
        let mut data: [u8; 1] = [0];
        let mut val: [u8; 1] = [0];
        loop {
            self.i2c
                .write_read(AXP202_SLAVE_ADDRESS, &[AXP202_LDO234_DC23_CTL], &mut data)
                .map_err(|_e| AXPError::ReadError)?;
            delay.delay_ms(1);
            if data[0] != 0 {
                break;
            }
        }

        match state {
            State::ON => data[0] |= 1 << channel,
            State::OFF => data[0] &= !(1 << channel),
        };

        if self.chip_id == AXP202_CHIP_ID {
            data[0] |= 1 << AXP202_DCDC3;
        }
        self.write(AXP202_SLAVE_ADDRESS, AXP202_LDO234_DC23_CTL, data[0])?;
        delay.delay_ms(1);
        self.i2c
            .write_read(AXP202_SLAVE_ADDRESS, &[AXP202_LDO234_DC23_CTL], &mut val)
            .map_err(|_e| AXPError::ReadError)?;
        if data == val {
            Ok(())
        } else {
            Err(AXPError::WriteError)
        }
    }

    pub fn is_charging(&mut self) -> Result<bool, AXPError> {
        let mut reg: [u8; 1] = [0];
        if !self.init {
            return Err(AXPError::InvalidState);
        }
        self.i2c
            .write_read(AXP202_SLAVE_ADDRESS, &[AXP202_MODE_CHGSTATUS], &mut reg)
            .map_err(|_e| AXPError::ReadError)?;
        Ok(is_open!(reg[0], 6))
    }

    pub fn is_battery_connect(&mut self) -> Result<bool, AXPError> {
        let mut reg: [u8; 1] = [0];
        if !self.init {
            return Err(AXPError::InvalidState);
        }
        self.i2c
            .write_read(AXP202_SLAVE_ADDRESS, &[AXP202_MODE_CHGSTATUS], &mut reg)
            .map_err(|_e| AXPError::ReadError)?;
        Ok(is_open!(reg[0], 5))
    }

    pub fn get_battery_percentage(&mut self) -> Result<u8, AXPError> {
        if !self.init {
            return Err(AXPError::InvalidState);
        }
        if self.chip_id != AXP202_CHIP_ID {
            return Err(AXPError::UnsupportedOperation);
        }
        let mut val: [u8; 1] = [0];
        if self.is_battery_connect()? {
            return Ok(0);
        }
        self.i2c
            .write_read(AXP202_SLAVE_ADDRESS, &[AXP202_BATT_PERCENTAGE], &mut val)
            .map_err(|_e| AXPError::ReadError)?;
        if (val[0] & bit_mask!(7)) == 0 {
            return Ok(val[0] & (!bit_mask!(7)));
        }
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
