/*!
  # Serial Peripheral Interface
  To construct the SPI instances, use the `Spi::new` function.
  The pin parameter is a tuple containing `(miso, mosi, cs, sck)` which should be configured via `into_spi_miso, into_spi_mosi, into_spi_ss, into_spi_sclk`.
  CS is optional - so you can also pass a tuple containing `(miso, mosi, sck)`
  ## Initialisation example
  ```rust
    let miso = parts.pin4.into_spi_miso();
    let mosi = parts.pin5.into_spi_mosi();
    let ss = parts.pin2.into_spi_ss();
    let sclk = parts.pin3.into_spi_sclk();
    let mut spi = hal::spi::Spi::new(
        dp.SPI,
        (miso, mosi, ss, sclk),
        embedded_hal::spi::MODE_0,
        8_000_000u32.Hz(),
        clocks,
    );
  ```
*/

use bl702_pac::SPI;
use embedded_hal::spi::FullDuplex as FullDuplexZero;
pub use embedded_hal_alpha::spi::blocking::Transfer;
use embedded_hal_alpha::spi::nb::FullDuplex;
pub use embedded_hal_alpha::spi::Mode;
use embedded_time::rate::Hertz;

use crate::pac;

use crate::clock::Clocks;

/// SPI error
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Rx overflow occurred
    RxOverflow,
    /// Rx underflow occurred
    RxUnderflow,
    /// Tx overflow occurred
    TxOverflow,
    /// Tx underflow occurred
    TxUnderflow,
}

/// The bit format to send the data in
#[derive(Debug, Clone, Copy)]
pub enum SpiBitFormat {
    /// Least significant bit first
    LsbFirst,
    /// Most significant bit first
    MsbFirst,
}

#[allow(clippy::missing_safety_doc)]
/// MISO pins - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait MisoPin<SPI> {}

#[allow(clippy::missing_safety_doc)]
/// MOSI pins - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait MosiPin<SPI> {}

#[allow(clippy::missing_safety_doc)]
/// SS pins - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait SsPin<SPI> {}

#[allow(clippy::missing_safety_doc)]
/// SCLK pins - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait SclkPin<SPI> {}

#[allow(clippy::missing_safety_doc)]
/// Spi pins - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait Pins<SPI> {}

unsafe impl<MODE> MosiPin<pac::SPI> for crate::gpio::Pin0<MODE> {}
unsafe impl<MODE> MisoPin<pac::SPI> for crate::gpio::Pin1<MODE> {}
unsafe impl<MODE> SsPin<pac::SPI> for crate::gpio::Pin2<MODE> {}
unsafe impl<MODE> SclkPin<pac::SPI> for crate::gpio::Pin3<MODE> {}
unsafe impl<MODE> MosiPin<pac::SPI> for crate::gpio::Pin4<MODE> {}
unsafe impl<MODE> MisoPin<pac::SPI> for crate::gpio::Pin5<MODE> {}
unsafe impl<MODE> SsPin<pac::SPI> for crate::gpio::Pin6<MODE> {}
unsafe impl<MODE> SclkPin<pac::SPI> for crate::gpio::Pin7<MODE> {}
unsafe impl<MODE> MosiPin<pac::SPI> for crate::gpio::Pin8<MODE> {}
unsafe impl<MODE> MisoPin<pac::SPI> for crate::gpio::Pin9<MODE> {}
unsafe impl<MODE> SsPin<pac::SPI> for crate::gpio::Pin10<MODE> {}
unsafe impl<MODE> SclkPin<pac::SPI> for crate::gpio::Pin11<MODE> {}
unsafe impl<MODE> MosiPin<pac::SPI> for crate::gpio::Pin12<MODE> {}
unsafe impl<MODE> MisoPin<pac::SPI> for crate::gpio::Pin13<MODE> {}
unsafe impl<MODE> SsPin<pac::SPI> for crate::gpio::Pin14<MODE> {}
unsafe impl<MODE> SclkPin<pac::SPI> for crate::gpio::Pin15<MODE> {}
unsafe impl<MODE> MosiPin<pac::SPI> for crate::gpio::Pin16<MODE> {}
unsafe impl<MODE> MisoPin<pac::SPI> for crate::gpio::Pin17<MODE> {}
unsafe impl<MODE> SsPin<pac::SPI> for crate::gpio::Pin18<MODE> {}
unsafe impl<MODE> SclkPin<pac::SPI> for crate::gpio::Pin19<MODE> {}
unsafe impl<MODE> MosiPin<pac::SPI> for crate::gpio::Pin20<MODE> {}
unsafe impl<MODE> MisoPin<pac::SPI> for crate::gpio::Pin21<MODE> {}
unsafe impl<MODE> SsPin<pac::SPI> for crate::gpio::Pin22<MODE> {}
unsafe impl<MODE> SclkPin<pac::SPI> for crate::gpio::Pin23<MODE> {}
unsafe impl<MODE> MosiPin<pac::SPI> for crate::gpio::Pin24<MODE> {}
unsafe impl<MODE> MisoPin<pac::SPI> for crate::gpio::Pin25<MODE> {}
unsafe impl<MODE> SsPin<pac::SPI> for crate::gpio::Pin26<MODE> {}
unsafe impl<MODE> SclkPin<pac::SPI> for crate::gpio::Pin27<MODE> {}
unsafe impl<MODE> MosiPin<pac::SPI> for crate::gpio::Pin28<MODE> {}
unsafe impl<MODE> MisoPin<pac::SPI> for crate::gpio::Pin29<MODE> {}
unsafe impl<MODE> SsPin<pac::SPI> for crate::gpio::Pin30<MODE> {}
unsafe impl<MODE> SclkPin<pac::SPI> for crate::gpio::Pin31<MODE> {}

unsafe impl<MISO, MOSI, SS, SCLK> Pins<SPI> for (MISO, MOSI, SS, SCLK)
where
    MISO: MisoPin<SPI>,
    MOSI: MosiPin<SPI>,
    SS: SsPin<SPI>,
    SCLK: SclkPin<SPI>,
{
}

unsafe impl<MISO, MOSI, SCLK> Pins<SPI> for (MISO, MOSI, SCLK)
where
    MISO: MisoPin<SPI>,
    MOSI: MosiPin<SPI>,
    SCLK: SclkPin<SPI>,
{
}

/// A Serial Peripheral Interface
pub struct Spi<SPI, PINS> {
    spi: SPI,
    pins: PINS,
}

impl<PINS> Spi<pac::SPI, PINS>
where
    PINS: Pins<pac::SPI>,
{
    /**
      Constructs an SPI instance in 8bit dataframe mode.
      The pin parameter tuple (miso, mosi, cs, sck) needs to be configured accordingly.
      You can also omit `cs` to have manual control over `cs`.
      The frequency cannot be more than half of the spi clock frequency.
    */
    pub fn new(spi: SPI, pins: PINS, mode: Mode, freq: Hertz<u32>, clocks: Clocks) -> Self
    where
        PINS: Pins<pac::SPI>,
    {
        let glb = unsafe { &*pac::GLB::ptr() };

        glb.glb_parm.modify(|_r, w| {
            w.reg_spi_0_master_mode().set_bit()
            //.reg_spi_0_swap()
            //.set_bit()
        });

        // length of phase 0 and 1 (i.e. low / high values of SCLK)
        // needs to be divided by two
        let len = clocks.spi_clk().0 / freq.0 / 2;
        if len > 256 || len == 0 {
            panic!("Cannot reach the desired SPI frequency");
        }

        let len = (len - 1) as u8;
        spi.spi_prd_0.modify(|_r, w| unsafe {
            w.cr_spi_prd_s()
                .bits(len)
                .cr_spi_prd_p()
                .bits(len)
                .cr_spi_prd_d_ph_0()
                .bits(len)
                .cr_spi_prd_d_ph_1()
                .bits(len)
        });

        spi.spi_prd_1
            .modify(|_r, w| unsafe { w.cr_spi_prd_i().bits(len) });

        spi.spi_config.modify(|_, w| unsafe {
            w.cr_spi_sclk_pol()
                .bit(match mode.polarity {
                    embedded_hal_alpha::spi::Polarity::IdleLow => false,
                    embedded_hal_alpha::spi::Polarity::IdleHigh => true,
                })
                .cr_spi_sclk_ph()
                .bit(match mode.phase {
                    embedded_hal_alpha::spi::Phase::CaptureOnFirstTransition => true,
                    embedded_hal_alpha::spi::Phase::CaptureOnSecondTransition => false,
                })
                .cr_spi_m_cont_en()
                .clear_bit() // disable cont mode
                .cr_spi_frame_size()
                .bits(0) // 8 bit frames
                .cr_spi_s_en()
                .clear_bit() // not slave
                .cr_spi_m_en()
                .set_bit() // master
        });

        Spi { spi, pins }
    }

    pub fn release(self) -> (pac::SPI, PINS) {
        (self.spi, self.pins)
    }

    /// Select which frame format is used for data transfers
    pub fn bit_format(&mut self, format: SpiBitFormat) {
        match format {
            SpiBitFormat::LsbFirst => self
                .spi
                .spi_config
                .modify(|_, w| w.cr_spi_bit_inv().set_bit()),
            SpiBitFormat::MsbFirst => self
                .spi
                .spi_config
                .modify(|_, w| w.cr_spi_bit_inv().clear_bit()),
        }
    }

    /// Clear FIFOs
    pub fn clear_fifo(&mut self) {
        self.spi
            .spi_fifo_config_0
            .write(|w| w.rx_fifo_clr().set_bit().tx_fifo_clr().set_bit());
    }
}

impl<PINS> FullDuplex<u8> for Spi<pac::SPI, PINS>
where
    PINS: Pins<pac::SPI>,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        let spi_fifo_config_0 = self.spi.spi_fifo_config_0.read();

        if spi_fifo_config_0.rx_fifo_overflow().bit_is_set() {
            Err(nb::Error::Other(Error::RxOverflow))
        } else if spi_fifo_config_0.rx_fifo_underflow().bit_is_set() {
            Err(nb::Error::Other(Error::RxUnderflow))
        } else if self.spi.spi_fifo_config_1.read().rx_fifo_cnt().bits() == 0 {
            Err(nb::Error::WouldBlock)
        } else {
            Ok((self.spi.spi_fifo_rdata.read().bits() & 0xff) as u8)
        }
    }

    fn write(&mut self, data: u8) -> nb::Result<(), Self::Error> {
        let spi_fifo_config_0 = self.spi.spi_fifo_config_0.read();

        if spi_fifo_config_0.tx_fifo_overflow().bit_is_set() {
            Err(nb::Error::Other(Error::TxOverflow))
        } else if spi_fifo_config_0.tx_fifo_underflow().bit_is_set() {
            Err(nb::Error::Other(Error::TxUnderflow))
        } else if self.spi.spi_fifo_config_1.read().tx_fifo_cnt().bits() == 0 {
            Err(nb::Error::WouldBlock)
        } else {
            self.spi
                .spi_fifo_wdata
                .write(|w| unsafe { w.bits(data as u32) });

            Ok(())
        }
    }
}

impl<PINS> FullDuplexZero<u8> for Spi<pac::SPI, PINS>
where
    PINS: Pins<pac::SPI>,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        FullDuplex::read(self)
    }

    fn send(&mut self, data: u8) -> nb::Result<(), Self::Error> {
        FullDuplex::write(self, data)
    }
}

//TODO: Default marker traits are removed from e-h 1.0 alpha 5, must re-implement manually.
// We can still use them for e-h 0.2 though, so that makes life easy
impl<PINS> embedded_hal::blocking::spi::transfer::Default<u8> for Spi<pac::SPI, PINS> where
    PINS: Pins<pac::SPI>
{
}

// This is basically the default impl of spi::blocking::Transfer from e-h 0.2
impl<PINS> embedded_hal_alpha::spi::blocking::Transfer<u8> for Spi<pac::SPI, PINS>
where
    PINS: Pins<pac::SPI>,
{
    type Error = Error;

    fn transfer(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        for word in words.iter_mut() {
            nb::block!(FullDuplex::write(self, *word))?;
            *word = nb::block!(FullDuplex::read(self))?;
        }

        Ok(())
    }
}

impl<PINS> embedded_hal::blocking::spi::write::Default<u8> for Spi<pac::SPI, PINS> where
    PINS: Pins<pac::SPI>
{
}

// This is basically the default impl of spi::blocking::write from e-h 0.2
impl<PINS> embedded_hal_alpha::spi::blocking::Write<u8> for Spi<pac::SPI, PINS>
where
    PINS: Pins<pac::SPI>,
{
    type Error = Error;
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        for word in words {
            nb::block!(FullDuplex::write(self, *word))?;
            nb::block!(FullDuplex::read(self))?;
        }

        Ok(())
    }
}

impl<PINS> embedded_hal::blocking::spi::write_iter::Default<u8> for Spi<pac::SPI, PINS> where
    PINS: Pins<pac::SPI>
{
}

// This is basically the default impl of spi::blocking::write_iter from e-h 0.2
impl<PINS> embedded_hal_alpha::spi::blocking::WriteIter<u8> for Spi<pac::SPI, PINS>
where
    PINS: Pins<pac::SPI>,
{
    type Error = Error;
    fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
    where
        WI: IntoIterator<Item = u8>,
    {
        for word in words.into_iter() {
            nb::block!(FullDuplex::write(self, word))?;
            nb::block!(FullDuplex::read(self))?;
        }

        Ok(())
    }
}
