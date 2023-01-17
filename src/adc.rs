//! Analog to digital converter configuration.
//! According to CubeMx, all STM32F4 chips use the same ADC IP so this should be correct for all variants.

#![deny(missing_docs)]

/*
    Currently unused but this is the formula for using temperature calibration:
    Temperature in °C = (110-30) * (adc_sample - VtempCal30::get().read()) / (VtempCal110::get().read()-VtempCal30::get().read()) + 30
*/

use crate::{
    gpio::{self, Analog},
    pac,
};
use core::fmt;

/// Vref internal signal, used for calibration
pub struct Vref;

/// Vbat internal signal, used for monitoring the battery (if used)
pub struct Vbat;

/// Core temperature internal signal
pub struct Temperature;

/// Vssa Analog Ground
pub struct Vssa;

macro_rules! adc_pins {
    ($($pin:ty => ($adc:ident, $chan:expr)),+ $(,)*) => {
        $(
            impl embedded_hal::adc::Channel<pac::$adc> for $pin {
                type ID = u8;
                fn channel() -> u8 { $chan }
            }
        )+
    };
}

/// Contains types related to ADC configuration
pub mod config {

    /// Clock config for the ADC
    /// Check the datasheet for the maximum speed the ADC supports
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Clock {
        /// CLK divided by 1
        Div1,
        /// CLK divided by 4
        Div4,
        /// CLK divided by 8
        Div8,
        /// CLK divided by 12
        Div12,
        /// CLK divided by 16
        Div16,
        /// CLK divided by 20
        Div20,
        /// CLK divided by 24
        Div24,
        /// CLK divided by 32
        Div32,
    }

    impl From<Clock> for u8 {
        fn from(c: Clock) -> u8 {
            match c {
                Clock::Div1 => 0,
                Clock::Div4 => 1,
                Clock::Div8 => 2,
                Clock::Div12 => 3,
                Clock::Div16 => 4,
                Clock::Div20 => 5,
                Clock::Div24 => 6,
                Clock::Div32 => 7,
            }
        }
    }

    /// Clock config for the ADC
    /// Check the datasheet for the maximum speed the ADC supports
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum ClockSource {
        /// Audio PLL
        AudioPLL,
        /// XCLK 32MHz
        XClk,
    }

    impl From<ClockSource> for bool {
        fn from(c: ClockSource) -> bool {
            match c {
                ClockSource::AudioPLL => false,
                ClockSource::XClk => true,
            }
        }
    }

    /// Clock config for the ADC
    /// Check the datasheet for the maximum speed the ADC supports
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Reference {
        /// 3.2V
        V3V2,
        /// 2.0V
        V2V0,
    }
    impl From<Reference> for bool {
        fn from(c: Reference) -> bool {
            match c {
                Reference::V3V2 => false,
                Reference::V2V0 => true,
            }
        }
    }
    impl From<Reference> for u32 {
        fn from(c: Reference) -> u32 {
            match c {
                Reference::V3V2 => 3200,
                Reference::V2V0 => 2000,
            }
        }
    }

    /// Resolution to sample at
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Resolution {
        /// 12-bit 2MS/s, OSR=1
        Twelve2M,
        /// 14-bit 125kS/s, OSR=16
        Fourteen125k,
        /// 14-bit 31.25kS/s, OSR=64
        Fourteen31k25,
        /// 16-bit 15.625kS/s, OSR=128
        Sixteen15k565,
        /// 16-bit 7.8125kS/s, OSR=256
        Sixteen7k8125,
    }
    impl From<Resolution> for u8 {
        fn from(r: Resolution) -> u8 {
            match r {
                Resolution::Twelve2M => 0,
                Resolution::Fourteen125k => 1,
                Resolution::Fourteen31k25 => 2,
                Resolution::Sixteen15k565 => 3,
                Resolution::Sixteen7k8125 => 4,
            }
        }
    }

    /// Continuous mode enable/disable
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum Continuous {
        /// Single mode, continuous disabled
        Single,
        /// Continuous mode enabled
        Continuous,
    }
    impl From<Continuous> for bool {
        fn from(c: Continuous) -> bool {
            match c {
                Continuous::Single => false,
                Continuous::Continuous => true,
            }
        }
    }

    /// Configuration for the adc.
    /// There are some additional parameters on the adc peripheral that can be
    /// added here when needed but this covers several basic usecases.
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub struct AdcConfig {
        pub(crate) clock: Clock,
        pub(crate) clock_source: ClockSource,
        pub(crate) resolution: Resolution,
        pub(crate) continuous: Continuous,
        pub(crate) reference: Reference,
    }

    impl AdcConfig {
        /// change the clock field
        pub fn clock(mut self, clock: Clock) -> Self {
            self.clock = clock;
            self
        }
        /// change the clock source field
        pub fn clock_source(mut self, clock_source: ClockSource) -> Self {
            self.clock_source = clock_source;
            self
        }
        /// change the resolution field
        pub fn resolution(mut self, resolution: Resolution) -> Self {
            self.resolution = resolution;
            self
        }
        /// change the continuous field
        pub fn continuous(mut self, continuous: Continuous) -> Self {
            self.continuous = continuous;
            self
        }
        /// Specify the reference voltage for the ADC.
        pub fn reference(mut self, reference: Reference) -> Self {
            self.reference = reference;
            self
        }
    }

    impl Default for AdcConfig {
        fn default() -> Self {
            Self {
                clock: Clock::Div16,
                clock_source: ClockSource::XClk,
                resolution: Resolution::Twelve2M,
                continuous: Continuous::Single,
                reference: Reference::V3V2,
            }
        }
    }
}

/// Analog to Digital Converter
#[derive(Clone, Copy)]
pub struct Adc<AON> {
    /// Current config of the ADC, kept up to date by the various set methods
    config: config::AdcConfig,
    /// The adc peripheral
    adc_reg: AON,
    /// VDDA in millivolts calculated from the factory calibration and vrefint
    vref: u32,
    /// Exclusive limit for the sample value possible for the configured resolution.
    max_sample: u32,
}
impl<ADC> fmt::Debug for Adc<ADC> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Adc: {{ vref: {:?}, max_sample: {:?}, config: {:?}, ... }}",
            self.vref, self.max_sample, self.config
        )
    }
}

macro_rules! adc {
    // Note that only AON supports measurement of VREF, VBAT, and the internal temperature sensor.
    (additionals: AON => ($common_type:ident)) => {
        /// Calculates the system VDDA by sampling the internal VREF channel and comparing
        /// the result with the value stored at the factory.
        pub fn calibrate(&mut self) {
            /*self.enable();

            let vref_en = self.temperature_and_vref_enabled();
            if !vref_en {
                self.enable_temperature_and_vref();
            }

            let vref_cal = VrefCal::get().read();
            let vref_samp = self.read(&mut Vref).unwrap(); //This can't actually fail, it's just in a result to satisfy hal trait

            self.calibrated_vdda = (VDDA_CALIB * u32::from(vref_cal)) / u32::from(vref_samp);
            if !vref_en {
                self.disable_temperature_and_vref();
            }*/
        }

        /*/// Returns if the temp and vref internal channels are enabled
        pub fn temperature_and_vref_enabled(&mut self) -> bool {
            unsafe {
                let common = &(*pac::$common_type::ptr());
                common.ccr.read().tsvrefe().bit_is_set()
            }
        }*/
    };

    // Provide a stub implementation for ADCs that do not have a means of sampling VREF.
    (additionals: $adc_type:ident => ($common_type:ident)) => {
        fn calibrate(&mut self) {}
    };

    ($($adc_type:ident => ($constructor_fn_name:ident, $common_type:ident)),+ $(,)*) => {
        $(
            //impl SafePeripheralRead for Adc<pac::$adc_type> { }

            impl Adc<pac::$adc_type> {

                adc!(additionals: $adc_type => ($common_type));

                /// Enables the ADC clock, resets the peripheral (optionally), runs calibration and applies the supplied config
                /// # Arguments
                /// * `reset` - should a reset be performed. This is provided because on some devices multiple ADCs share the same common reset
                pub fn $constructor_fn_name(adc: pac::$adc_type, config: config::AdcConfig) -> Adc<pac::$adc_type> {
                    let mut s = Self {
                        config,
                        adc_reg: adc,
                        vref: config.reference.into(),
                        max_sample: 0,
                    };

                    //Probably unnecessary to disable the ADC in most cases but it shouldn't do any harm either
                    s.disable();
                    s.apply_config(config);

                    s.enable();
                    //s.calibrate();

                    s
                }

                /// Applies all fields in AdcConfig
                pub fn apply_config(&mut self, config: config::AdcConfig) {
                    self.set_clock(config.clock);
                    self.set_clock_source(config.clock_source);
                    self.set_resolution(config.resolution);
                    self.set_continuous(config.continuous);
                    self.set_reference(config.reference);
                }

                /// Returns if the adc is enabled
                pub fn is_enabled(&self) -> bool {
                    self.adc_reg.gpadc_reg_cmd.read().gpadc_global_en().bit_is_set()
                }

                /// Enables the adc
                pub fn enable(&mut self) {
                    self.adc_reg.gpadc_reg_cmd.modify(|_, w| w.gpadc_global_en().set_bit());
                }

                /// Disables the adc
                /// # Note
                /// The ADC in the f4 has few restrictions on what can be configured while the ADC
                /// is enabled. If any bugs are found where some settings aren't "sticking" try disabling
                /// the ADC before changing them. The reference manual for the chip I'm using only states
                /// that the sequence registers are locked when they are being converted.
                pub fn disable(&mut self) {
                    self.adc_reg.gpadc_reg_cmd.modify(|_, w| w.gpadc_global_en().clear_bit());
                }

                /// Starts conversion sequence. Waits for the hardware to indicate it's actually started.
                pub fn start_conversion(&mut self) {
                    self.enable();

                    //stop previous conversion
                    self.adc_reg.gpadc_reg_cmd.modify(|_, w| w.gpadc_conv_start().clear_bit());
                    self.clear_end_of_conversion_flag();

                    //Start conversion
                    self.adc_reg.gpadc_reg_cmd.modify(|_, w| w.gpadc_conv_start().set_bit());

                    while !self.adc_reg.gpadc_reg_cmd.read().gpadc_conv_start().bit_is_set() {}
                }

                /// Sets the clock for the adc
                pub fn set_clock(&mut self, clock: config::Clock) {
                    self.config.clock = clock;
                    self.adc_reg.gpadc_reg_config1.modify(|_, w| unsafe {w.gpadc_clk_div_ratio().bits(clock.into())});
                }

                /// Sets the clock source for the adc
                pub fn set_clock_source(&mut self, clock_source: config::ClockSource) {
                    self.config.clock_source = clock_source;
                    unsafe {
                        let glb = &(*pac::GLB::ptr());
                        glb.cgen_cfg1.modify(|_, w| w.gpip().clear_bit()); // disable clock to change settings
                        glb.gpadc_32m_src_ctrl.modify(|_, w| w
                            .gpadc_32m_clk_sel().bit(clock_source.into())
                            .gpadc_32m_clk_div().bits(0)
                            .gpadc_32m_div_en().set_bit());
                        glb.cgen_cfg1.modify(|_, w| w.gpip().set_bit()); // should be por default
                    }
                }

                /// Sets the sampling resolution
                pub fn set_resolution(&mut self, resolution: config::Resolution) {
                    self.max_sample = match resolution {
                        config::Resolution::Twelve2M => (1 << 12),
                        config::Resolution::Fourteen125k => (1 << 14),
                        config::Resolution::Fourteen31k25 => (1 << 14),
                        config::Resolution::Sixteen15k565 => (1 << 16),
                        config::Resolution::Sixteen7k8125 => (1 << 16),
                    };
                    self.config.resolution = resolution;
                    self.adc_reg.gpadc_reg_config1.modify(|_, w| unsafe {w.gpadc_res_sel().bits(resolution.into())});
                }


                /// Enables and disables continuous mode
                pub fn set_continuous(&mut self, continuous: config::Continuous) {
                    self.config.continuous = continuous;
                    self.adc_reg.gpadc_reg_config1.modify(|_, w| w.gpadc_cont_conv_en().bit(continuous.into()));
                }

                /// Sets ADC Reference
                pub fn set_reference(&mut self, reference: config::Reference) {
                    self.config.reference = reference;
                    self.vref = reference.into();
                    self.adc_reg.gpadc_reg_config2.modify(|_, w| w.gpadc_vref_sel().bit(reference.into()));
                }

                /// Resets the end-of-conversion flag
                pub fn clear_fifo(&mut self) {
                    unsafe {
                        let gpip = &(*pac::$common_type::ptr());
                        gpip.gpadc_config.modify(|_, w| w
                            .gpadc_fifo_clr().set_bit());
                    }
                }

                /// Resets the end-of-conversion flag
                pub fn clear_end_of_conversion_flag(&mut self) {
                    unsafe {
                        let gpip = &(*pac::$common_type::ptr());
                        gpip.gpadc_config.modify(|_, w| w
                            .gpadc_rdy_clr().set_bit());
                    }
                }

                /// Configure a channel for sampling.

                /// # Arguments
                /// * `channel` - channel to configure
                /// to sample for at a given ADC clock frequency
                pub fn configure_channel<CHANNEL>(&mut self, _channel: &CHANNEL)
                where
                    CHANNEL: embedded_hal::adc::Channel<pac::$adc_type, ID=u8>
                {
                    let channel = CHANNEL::channel();

                    self.adc_reg.gpadc_reg_cmd.modify(|_, w| unsafe { w
                        .gpadc_pos_sel().bits(channel)
                        .gpadc_neg_sel().bits(23)
                        .gpadc_neg_gnd().set_bit()
                        }
                    );
                }

                /// Returns the current sample stored in the ADC data register
                pub fn current_sample(&self) -> u16 {
                    let gpip = unsafe { &(*pac::$common_type::ptr()) };
                    let sample = gpip.gpadc_dma_rdata.read().gpadc_dma_rdata().bits() as u16;
                    let shift = match self.config.resolution {
                        config::Resolution::Twelve2M => 4,
                        config::Resolution::Fourteen125k => 2,
                        config::Resolution::Fourteen31k25 => 2,
                        config::Resolution::Sixteen15k565 => 0,
                        config::Resolution::Sixteen7k8125 => 0,
                    };
                    sample >> shift
                }

                /// Return an option for current fifo data
                pub fn try_read(&self) -> Option<u16> {
                    let gpip = unsafe { &(*pac::$common_type::ptr()) };
                    if gpip.gpadc_config.read().gpadc_fifo_ne().bit_is_set() {
                        Some(self.current_sample())
                    } else {
                        None
                    }
                }

                /// Converts a sample value to millivolts using calibrated VDDA and configured resolution.
                /// Due to the ADC characteristics VDDA will never be reached as described in #362 and
                pub fn sample_to_millivolts(&self, sample: u16) -> u16 {
                    ((u32::from(sample) * self.vref) / self.max_sample) as u16
                }

                /// Make a converter for samples to millivolts
                pub fn make_sample_to_millivolts(&self) -> impl Fn(u16)->u16 {
                    let vref = self.vref;
                    let max_sample=self.max_sample;
                    move |sample| {
                        ((u32::from(sample) * vref) / max_sample) as u16
                    }
                }

                /// Returns the VDDA in millivolts calculated from the factory calibration and vrefint. Can be used to get calibration data from AON and use it to configure ADCs that don't support calibration.
                pub fn reference_voltage(&self) -> u32 {
                    self.vref
                }

                /// Block until the conversion is completed
                /// # Panics
                /// Will panic if there is no conversion started and the end-of-conversion bit is not set
                pub fn wait_for_conversion_sequence(&self) {
                    unsafe {
                        let gpip = &(*pac::$common_type::ptr());
                        if !self.adc_reg.gpadc_reg_cmd.read().gpadc_conv_start().bit_is_set() && !gpip.gpadc_config.read().gpadc_fifo_ne().bit_is_set() {
                            panic!("Waiting for end-of-conversion but no conversion started");
                        }
                        while !gpip.gpadc_config.read().gpadc_fifo_ne().bit_is_set() {}
                        //Clear the conversion started flag
                        self.adc_reg.gpadc_reg_cmd.modify(|_, w| w.gpadc_conv_start().clear_bit());
                    }
                }

                /// Synchronously convert a single sample
                /// Note that it reconfigures the adc sequence and doesn't restore it
                pub fn convert<PIN>(&mut self, pin: &PIN) -> u16
                where
                    PIN: embedded_hal::adc::Channel<pac::$adc_type, ID=u8>
                {
                    if self.adc_reg.gpadc_reg_config1.read().gpadc_cont_conv_en().bit_is_set() {
                        self.disable();
                        self.adc_reg.gpadc_reg_config1.modify(|_, w| w.gpadc_cont_conv_en().clear_bit());
                    }

                    self.configure_channel(pin);
                    self.enable();
                    self.clear_fifo();
                    self.clear_end_of_conversion_flag();
                    self.start_conversion();

                    //Wait for the sequence to complete
                    self.wait_for_conversion_sequence();

                    let result = self.current_sample();

                    //Reset the config
                    self.apply_config(self.config);

                    result
                }
            }

            impl Adc<pac::$adc_type> {
                fn read<PIN>(&mut self, pin: &mut PIN) -> nb::Result<u16, ()>
                    where PIN: embedded_hal::adc::Channel<pac::$adc_type, ID=u8>,
                {
                    let enabled = self.is_enabled();
                    if !enabled {
                        self.enable();
                    }

                    let sample = self.convert(pin);

                    if !enabled {
                        self.disable();
                    }

                    Ok(sample)
                }
            }

            impl<PIN> embedded_hal::adc::OneShot<pac::$adc_type, u16, PIN> for Adc<pac::$adc_type>
            where
                PIN: embedded_hal::adc::Channel<pac::$adc_type, ID=u8>,
            {
                type Error = ();

                fn read(&mut self, pin: &mut PIN) -> nb::Result<u16, Self::Error> {
                    self.read::<PIN>(pin)
                }
            }
        )+
    };
}

adc!(AON => (adc, GPIP));

adc_pins!(
    gpio::Pin8<Analog> => (AON, 0),
    gpio::Pin15<Analog> => (AON, 1),
    gpio::Pin17<Analog> => (AON, 2),
    gpio::Pin11<Analog> => (AON, 3),
    gpio::Pin12<Analog> => (AON, 4),
    gpio::Pin14<Analog> => (AON, 5),
    gpio::Pin7<Analog> => (AON, 6),
    gpio::Pin9<Analog> => (AON, 7),
    gpio::Pin18<Analog> => (AON, 8),
    gpio::Pin19<Analog> => (AON, 9),
    gpio::Pin20<Analog> => (AON, 10),
    gpio::Pin21<Analog> => (AON, 11),
    // daca
    // dacb
    Temperature => (AON, 14),
    Vref => (AON, 16),
    Vbat => (AON, 18),
    Vssa => (AON, 23)
);
