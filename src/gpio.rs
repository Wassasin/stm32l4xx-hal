//! General Purpose Input / Output

// Based on
// https://github.com/japaric/stm32f30x-hal/blob/master/src/gpio.rs

use core::marker::PhantomData;

use crate::rcc::AHB2;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, ahb: &mut AHB2) -> Self::Parts;
}

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;
/// Pulled down input (type state)
pub struct PullDown;
/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output (type state)
pub struct PushPull;
/// Open drain output (type state)
pub struct OpenDrain;

/// Alternate mode (type state)
pub struct Alternate<AF, MODE>
{
    _af: PhantomData<AF>,
    _mode: PhantomData<MODE>,
}

pub enum State {
    High,
    Low,
}

/// Alternate function 0 (type state)
pub struct AF0;

/// Alternate function 1 (type state)
pub struct AF1;

/// Alternate function 2 (type state)
pub struct AF2;

/// Alternate function 3 (type state)
pub struct AF3;

/// Alternate function 4 (type state)
pub struct AF4;

/// Alternate function 5 (type state)
pub struct AF5;

/// Alternate function 6 (type state)
pub struct AF6;

/// Alternate function 7 (type state)
pub struct AF7;

/// Alternate function 8 (type state)
pub struct AF8;

/// Alternate function 9 (type state)
pub struct AF9;

/// Alternate function 10 (type state)
pub struct AF10;

/// Alternate function 11 (type state)
pub struct AF11;

/// Alternate function 12 (type state)
pub struct AF12;

/// Alternate function 13 (type state)
pub struct AF13;

/// Alternate function 14 (type state)
pub struct AF14;

/// Alternate function 15 (type state)
pub struct AF15;

macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

macro_rules! impl_into_af {
    ($PXi:ident $AFR:ident $i:expr, $(($AF:ident, $NUM:expr, $NAME:ident));* $(;)?) => {
        $(
            doc_comment! {
                concat!("Configures the pin to serve as alternate function ", stringify!($NUM), " (", stringify!($AF), ")"),
                pub fn $NAME(self, moder: &mut MODER, afr: &mut $AFR) -> $PXi<Alternate<$AF, MODE>> {
                    const OFF_MODE: u32 = 2 * $i;
                    const OFF_AFR: u32 = 4 * ($i % 8);
                    const MODE: u32 = 0b10; // alternate function mode

                    moder.moder().modify(|r, w| unsafe {
                        w.bits((r.bits() & !(0b11 << OFF_MODE)) | (MODE << OFF_MODE))
                    });
                    afr.afr().modify(|r, w| unsafe {
                        w.bits((r.bits() & !(0b1111 << OFF_AFR)) | ($NUM << OFF_AFR))
                    });

                    $PXi { _mode: PhantomData }
                }
            }
        )*
    }
}

// In general, each parameter should use the same identifying letter. The third parameter, $gpioy,
// is an exception: it refers to the path to the RegisterBlock trait, which is sometimes reused. To
// find out which $gpioy to use, search in the stm32l4 documentation for the GPIOX struct, click on
// the RegisterBlock return value of the ptr() method, and check which gpioy is in its ::-path.
macro_rules! gpio {
    ($GPIOX:ident, $gpiox:ident, $gpioy:ident, $iopxenr:ident, $iopxrst:ident, $PXx:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty, $AFR:ident),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use core::marker::PhantomData;
            use core::convert::Infallible;

            use crate::hal::digital::v2::{OutputPin, InputPin};
            use crate::stm32::{$gpioy, $GPIOX};

            use crate::rcc::AHB2;
            use super::{
                Alternate,
                AF1, AF2, AF3, AF4, AF5, AF6, AF7, AF8, AF9, AF10, AF11, AF12, AF13, AF14, AF15,
                Floating, GpioExt, Input, OpenDrain, Output,
                PullDown, PullUp, PushPull, State,
            };

            /// GPIO parts
            pub struct Parts {
                /// Opaque AFRH register
                pub afrh: AFRH,
                /// Opaque AFRL register
                pub afrl: AFRL,
                /// Opaque MODER register
                pub moder: MODER,
                /// Opaque OTYPER register
                pub otyper: OTYPER,
                /// Opaque PUPDR register
                pub pupdr: PUPDR,
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self, ahb: &mut AHB2) -> Parts {
                    ahb.enr().modify(|_, w| w.$iopxenr().set_bit());
                    ahb.rstr().modify(|_, w| w.$iopxrst().set_bit());
                    ahb.rstr().modify(|_, w| w.$iopxrst().clear_bit());

                    Parts {
                        afrh: AFRH { _0: () },
                        afrl: AFRL { _0: () },
                        moder: MODER { _0: () },
                        otyper: OTYPER { _0: () },
                        pupdr: PUPDR { _0: () },
                        $(
                            $pxi: $PXi { _mode: PhantomData },
                        )+
                    }
                }
            }

            /// Opaque AFRL register
            pub struct AFRL {
                _0: (),
            }

            impl AFRL {
                pub(crate) fn afr(&mut self) -> &$gpioy::AFRL {
                    unsafe { &(*$GPIOX::ptr()).afrl }
                }
            }

            /// Opaque AFRH register
            pub struct AFRH {
                _0: (),
            }

            impl AFRH {
                pub(crate) fn afr(&mut self) -> &$gpioy::AFRH {
                    unsafe { &(*$GPIOX::ptr()).afrh }
                }
            }

            /// Opaque MODER register
            pub struct MODER {
                _0: (),
            }

            impl MODER {
                pub(crate) fn moder(&mut self) -> &$gpioy::MODER {
                    unsafe { &(*$GPIOX::ptr()).moder }
                }
            }

            /// Opaque OTYPER register
            pub struct OTYPER {
                _0: (),
            }

            impl OTYPER {
                pub(crate) fn otyper(&mut self) -> &$gpioy::OTYPER {
                    unsafe { &(*$GPIOX::ptr()).otyper }
                }
            }

            /// Opaque PUPDR register
            pub struct PUPDR {
                _0: (),
            }

            impl PUPDR {
                pub(crate) fn pupdr(&mut self) -> &$gpioy::PUPDR {
                    unsafe { &(*$GPIOX::ptr()).pupdr }
                }
            }

            /// Partially erased pin
            pub struct $PXx<MODE> {
                i: u8,
                _mode: PhantomData<MODE>,
            }

            impl<MODE> OutputPin for $PXx<Output<MODE>> {
                type Error = Infallible;

                fn set_high(&mut self) -> Result<(), Self::Error> {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << self.i)) }
                    Ok(())
                }

                fn set_low(&mut self) -> Result<(), Self::Error> {
                    // NOTE(unsafe) atomic write to a stateless register
                    unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + self.i))) }
                    Ok(())
                }
            }

            $(
                /// Pin
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin to operate as a floating input pin
                    pub fn into_floating_input(
                        self,
                        moder: &mut MODER,
                        pupdr: &mut PUPDR,
                    ) -> $PXi<Input<Floating>> {
                        let offset = 2 * $i;

                        // input mode
                        moder
                            .moder()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                        // no pull-up or pull-down
                        pupdr
                            .pupdr()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled down input pin
                    pub fn into_pull_down_input(
                        self,
                        moder: &mut MODER,
                        pupdr: &mut PUPDR,
                    ) -> $PXi<Input<PullDown>> {
                        let offset = 2 * $i;

                        // input mode
                        moder
                            .moder()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                        // pull-down
                        pupdr.pupdr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (0b10 << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as a pulled up input pin
                    pub fn into_pull_up_input(
                        self,
                        moder: &mut MODER,
                        pupdr: &mut PUPDR,
                    ) -> $PXi<Input<PullUp>> {
                        let offset = 2 * $i;

                        // input mode
                        moder
                            .moder()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b11 << offset)) });

                        // pull-up
                        pupdr.pupdr().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset))
                        });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an open drain output pin
                    pub fn into_open_drain_output(
                        self,
                        moder: &mut MODER,
                        otyper: &mut OTYPER,
                    ) -> $PXi<Output<OpenDrain>> {
                        let offset = 2 * $i;

                        // general purpose output mode
                        let mode = 0b01;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                        });

                        // open drain output
                        otyper
                            .otyper()
                            .modify(|r, w| unsafe { w.bits(r.bits() | (0b1 << $i)) });

                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin to operate as an push pull output pin
                    /// Initial state will be low
                    pub fn into_push_pull_output(
                        self,
                        moder: &mut MODER,
                        otyper: &mut OTYPER,
                    ) -> $PXi<Output<PushPull>> {
                        self.into_push_pull_output_with_state(moder, otyper, State::Low)
                    }

                    /// Configures the pin to operate as an push pull output pin
                    /// Initial state can be chosen to be high or low
                    pub fn into_push_pull_output_with_state(
                        self,
                        moder: &mut MODER,
                        otyper: &mut OTYPER,
                        initial_state: State,
                    ) -> $PXi<Output<PushPull>> {
                        let mut res = $PXi { _mode: PhantomData };

                        // set pin high/low before activating, to prevent
                        // spurious signals (e.g. LED flash)
                        // TODO: I still see a flash of LED using this order
                        match initial_state {
                            State::High => res.set_high().unwrap(),
                            State::Low => res.set_low().unwrap(),
                        }

                        let offset = 2 * $i;

                        // general purpose output mode
                        let mode = 0b01;
                        moder.moder().modify(|r, w| unsafe {
                            w.bits((r.bits() & !(0b11 << offset)) | (mode << offset))
                        });

                        // push pull output
                        otyper
                            .otyper()
                            .modify(|r, w| unsafe { w.bits(r.bits() & !(0b1 << $i)) });

                        res
                    }

                    /// Configures the pin to operate as an touch sample
                    pub fn into_touch_sample(
                        self,
                        moder: &mut MODER,
                        otyper: &mut OTYPER,
                        afr: &mut $AFR,
                    ) -> $PXi<Alternate<AF9, Output<OpenDrain>>> {
                        let od = self.into_open_drain_output(moder, otyper);
                        od.into_af9(moder, afr)
                    }

                    /// Configures the pin to operate as an touch channel
                    pub fn into_touch_channel(
                        self,
                        moder: &mut MODER,
                        otyper: &mut OTYPER,
                        afr: &mut $AFR,
                    ) -> $PXi<Alternate<AF9, Output<PushPull>>> {
                        let od = self.into_push_pull_output(moder, otyper);
                        od.into_af9(moder, afr)
                    }
                }

                impl $PXi<Output<OpenDrain>> {
                    /// Enables / disables the internal pull up
                    pub fn internal_pull_up(&mut self, pupdr: &mut PUPDR, on: bool) {
                        let offset = 2 * $i;

                        pupdr.pupdr().modify(|r, w| unsafe {
                            w.bits(
                                (r.bits() & !(0b11 << offset)) | if on {
                                    0b01 << offset
                                } else {
                                    0
                                },
                            )
                        });
                    }
                }

                impl<MODE> $PXi<Output<MODE>> {
                    /// Erases the pin number from the type
                    ///
                    /// This is useful when you want to collect the pins into an array where you
                    /// need all the elements to have the same type
                    pub fn downgrade(self) -> $PXx<Output<MODE>> {
                        $PXx {
                            i: $i,
                            _mode: self._mode,
                        }
                    }
                }

                impl<MODE> OutputPin for $PXi<Output<MODE>> {
                    type Error = Infallible;
                    
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << $i)) }
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        // NOTE(unsafe) atomic write to a stateless register
                        unsafe { (*$GPIOX::ptr()).bsrr.write(|w| w.bits(1 << (16 + $i))) }
                        Ok(())
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>> {
                    type Error = Infallible;

                    fn is_high(&self) -> Result<bool, Self::Error> {
                        Ok(!self.is_low().unwrap())
                    }

                    fn is_low(&self) -> Result<bool, Self::Error> {
                        // NOTE(unsafe) atomic read with no side effects
                        Ok(unsafe { (*$GPIOX::ptr()).idr.read().bits() & (1 << $i) == 0 })
                    }
                }

                impl<MODE> $PXi<MODE> {
                    impl_into_af! {
                        $PXi $AFR $i,
                        (AF1, 1, into_af1);
                        (AF2, 2, into_af2);
                        (AF3, 3, into_af3);
                        (AF4, 4, into_af4);
                        (AF5, 5, into_af5);
                        (AF6, 6, into_af6);
                        (AF7, 7, into_af7);
                        (AF8, 8, into_af8);
                        (AF9, 9, into_af9);
                        (AF10, 10, into_af10);
                        (AF11, 11, into_af11);
                        (AF12, 12, into_af12);
                        (AF13, 13, into_af13);
                        (AF14, 14, into_af14);
                        (AF15, 15, into_af15);
                    }
                }
            )+
        }
    }
}

gpio!(GPIOA, gpioa, gpioa, gpioaen, gpioarst, PAx, [
    PA0: (pa0, 0, Input<Floating>, AFRL),
    PA1: (pa1, 1, Input<Floating>, AFRL),
    PA2: (pa2, 2, Input<Floating>, AFRL),
    PA3: (pa3, 3, Input<Floating>, AFRL),
    PA4: (pa4, 4, Input<Floating>, AFRL),
    PA5: (pa5, 5, Input<Floating>, AFRL),
    PA6: (pa6, 6, Input<Floating>, AFRL),
    PA7: (pa7, 7, Input<Floating>, AFRL),
    PA8: (pa8, 8, Input<Floating>, AFRH),
    PA9: (pa9, 9, Input<Floating>, AFRH),
    PA10: (pa10, 10, Input<Floating>, AFRH),
    PA11: (pa11, 11, Input<Floating>, AFRH),
    PA12: (pa12, 12, Input<Floating>, AFRH),
    PA13: (pa13, 13, Input<Floating>, AFRH),
    PA14: (pa14, 14, Input<Floating>, AFRH),
    PA15: (pa15, 15, Input<Floating>, AFRH),
]);

gpio!(GPIOB, gpiob, gpiob, gpioben, gpiobrst, PBx, [
    PB0: (pb0, 0, Input<Floating>, AFRL),
    PB1: (pb1, 1, Input<Floating>, AFRL),
    PB2: (pb2, 2, Input<Floating>, AFRL),
    PB3: (pb3, 3, Input<Floating>, AFRL),
    PB4: (pb4, 4, Input<Floating>, AFRL),
    PB5: (pb5, 5, Input<Floating>, AFRL),
    PB6: (pb6, 6, Input<Floating>, AFRL),
    PB7: (pb7, 7, Input<Floating>, AFRL),
    PB8: (pb8, 8, Input<Floating>, AFRH),
    PB9: (pb9, 9, Input<Floating>, AFRH),
    PB10: (pb10, 10, Input<Floating>, AFRH),
    PB11: (pb11, 11, Input<Floating>, AFRH),
    PB12: (pb12, 12, Input<Floating>, AFRH),
    PB13: (pb13, 13, Input<Floating>, AFRH),
    PB14: (pb14, 14, Input<Floating>, AFRH),
    PB15: (pb15, 15, Input<Floating>, AFRH),
]);

gpio!(GPIOC, gpioc, gpioc, gpiocen, gpiocrst, PCx, [
    PC0: (pc0, 0, Input<Floating>, AFRL),
    PC1: (pc1, 1, Input<Floating>, AFRL),
    PC2: (pc2, 2, Input<Floating>, AFRL),
    PC3: (pc3, 3, Input<Floating>, AFRL),
    PC4: (pc4, 4, Input<Floating>, AFRL),
    PC5: (pc5, 5, Input<Floating>, AFRL),
    PC6: (pc6, 6, Input<Floating>, AFRL),
    PC7: (pc7, 7, Input<Floating>, AFRL),
    PC8: (pc8, 8, Input<Floating>, AFRH),
    PC9: (pc9, 9, Input<Floating>, AFRH),
    PC10: (pc10, 10, Input<Floating>, AFRH),
    PC11: (pc11, 11, Input<Floating>, AFRH),
    PC12: (pc12, 12, Input<Floating>, AFRH),
    PC13: (pc13, 13, Input<Floating>, AFRH),
    PC14: (pc14, 14, Input<Floating>, AFRH),
    PC15: (pc15, 15, Input<Floating>, AFRH),
]);

gpio!(GPIOD, gpiod, gpioc, gpioden, gpiodrst, PDx, [
    PD0: (pd0, 0, Input<Floating>, AFRL),
    PD1: (pd1, 1, Input<Floating>, AFRL),
    PD2: (pd2, 2, Input<Floating>, AFRL),
    PD3: (pd3, 3, Input<Floating>, AFRL),
    PD4: (pd4, 4, Input<Floating>, AFRL),
    PD5: (pd5, 5, Input<Floating>, AFRL),
    PD6: (pd6, 6, Input<Floating>, AFRL),
    PD7: (pd7, 7, Input<Floating>, AFRL),
    PD8: (pd8, 8, Input<Floating>, AFRH),
    PD9: (pd9, 9, Input<Floating>, AFRH),
    PD10: (pd10, 10, Input<Floating>, AFRH),
    PD11: (pd11, 11, Input<Floating>, AFRH),
    PD12: (pd12, 12, Input<Floating>, AFRH),
    PD13: (pd13, 13, Input<Floating>, AFRH),
    PD14: (pd14, 14, Input<Floating>, AFRH),
    PD15: (pd15, 15, Input<Floating>, AFRH),
]);

gpio!(GPIOE, gpioe, gpioc, gpioeen, gpioerst, PEx, [
    PE0: (pe0, 0, Input<Floating>, AFRL),
    PE1: (pe1, 1, Input<Floating>, AFRL),
    PE2: (pe2, 2, Input<Floating>, AFRL),
    PE3: (pe3, 3, Input<Floating>, AFRL),
    PE4: (pe4, 4, Input<Floating>, AFRL),
    PE5: (pe5, 5, Input<Floating>, AFRL),
    PE6: (pe6, 6, Input<Floating>, AFRL),
    PE7: (pe7, 7, Input<Floating>, AFRL),
    PE8: (pe8, 8, Input<Floating>, AFRH),
    PE9: (pe9, 9, Input<Floating>, AFRH),
    PE10: (pe10, 10, Input<Floating>, AFRH),
    PE11: (pe11, 11, Input<Floating>, AFRH),
    PE12: (pe12, 12, Input<Floating>, AFRH),
    PE13: (pe13, 13, Input<Floating>, AFRH),
    PE14: (pe14, 14, Input<Floating>, AFRH),
    PE15: (pe15, 15, Input<Floating>, AFRH),
]);

#[cfg(any(
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
gpio!(GPIOF, gpiof, gpioc, gpiofen, gpiofrst, PFx, [
    PF0: (pf0, 0, Input<Floating>, AFRL),
    PF1: (pf1, 1, Input<Floating>, AFRL),
    PF2: (pf2, 2, Input<Floating>, AFRL),
    PF3: (pf3, 3, Input<Floating>, AFRL),
    PF4: (pf4, 4, Input<Floating>, AFRL),
    PF5: (pf5, 5, Input<Floating>, AFRL),
    PF6: (pf6, 6, Input<Floating>, AFRL),
    PF7: (pf7, 7, Input<Floating>, AFRL),
    PF8: (pf8, 8, Input<Floating>, AFRH),
    PF9: (pf9, 9, Input<Floating>, AFRH),
    PF10: (pf10, 10, Input<Floating>, AFRH),
    PF11: (pf11, 11, Input<Floating>, AFRH),
    PF12: (pf12, 12, Input<Floating>, AFRH),
    PF13: (pf13, 13, Input<Floating>, AFRH),
    PF14: (pf14, 14, Input<Floating>, AFRH),
    PF15: (pf15, 15, Input<Floating>, AFRH),
]);

#[cfg(any(
    feature = "stm32l4x5",
    feature = "stm32l4x6"
))]
gpio!(GPIOG, gpiog, gpioc, gpiogen, gpiogrst, PGx, [
    PG0: (pg0, 0, Input<Floating>, AFRL),
    PG1: (pg1, 1, Input<Floating>, AFRL),
    PG2: (pg2, 2, Input<Floating>, AFRL),
    PG3: (pg3, 3, Input<Floating>, AFRL),
    PG4: (pg4, 4, Input<Floating>, AFRL),
    PG5: (pg5, 5, Input<Floating>, AFRL),
    PG6: (pg6, 6, Input<Floating>, AFRL),
    PG7: (pg7, 7, Input<Floating>, AFRL),
    PG8: (pg8, 8, Input<Floating>, AFRH),
    PG9: (pg9, 9, Input<Floating>, AFRH),
    PG10: (pg10, 10, Input<Floating>, AFRH),
    PG11: (pg11, 11, Input<Floating>, AFRH),
    PG12: (pg12, 12, Input<Floating>, AFRH),
    PG13: (pg13, 13, Input<Floating>, AFRH),
    PG14: (pg14, 14, Input<Floating>, AFRH),
    PG15: (pg15, 15, Input<Floating>, AFRH),
]);
