use core::{convert::Infallible, marker::PhantomData};

use bao1x_pac::Iox;

use crate::gpio::port::{Port, PortA, PortB, PortC, PortD, PortE};

pub(crate) trait Sealed {}

#[allow(private_bounds)]
pub trait PinId: Sealed {
    type Port: Port;
    const INDEX: usize;
}

macro_rules! def_pinid {
    ($name:ident, $port:ty, $index:literal) => {
        pub struct $name {}

        impl Sealed for $name {}

        impl PinId for $name {
            type Port = $port;
            const INDEX: usize = $index;
        }
    };
}

def_pinid!(PA00, PortA, 0);
def_pinid!(PA01, PortA, 1);
def_pinid!(PA02, PortA, 2);
def_pinid!(PA03, PortA, 3);
def_pinid!(PA04, PortA, 4);
def_pinid!(PA05, PortA, 5);
def_pinid!(PA06, PortA, 6);
def_pinid!(PA07, PortA, 7);

def_pinid!(PB00, PortB, 0);
def_pinid!(PB01, PortB, 1);
def_pinid!(PB02, PortB, 2);
def_pinid!(PB03, PortB, 3);
def_pinid!(PB04, PortB, 4);
def_pinid!(PB05, PortB, 5);
def_pinid!(PB06, PortB, 6);
def_pinid!(PB07, PortB, 7);
def_pinid!(PB08, PortB, 8);
def_pinid!(PB09, PortB, 9);
def_pinid!(PB10, PortB, 10);
def_pinid!(PB11, PortB, 11);
def_pinid!(PB12, PortB, 12);
def_pinid!(PB13, PortB, 13);
def_pinid!(PB14, PortB, 14);
def_pinid!(PB15, PortB, 15);

def_pinid!(PC00, PortC, 0);
def_pinid!(PC01, PortC, 1);
def_pinid!(PC02, PortC, 2);
def_pinid!(PC03, PortC, 3);
def_pinid!(PC04, PortC, 4);
def_pinid!(PC05, PortC, 5);
def_pinid!(PC06, PortC, 6);
def_pinid!(PC07, PortC, 7);
def_pinid!(PC08, PortC, 8);
def_pinid!(PC09, PortC, 9);
def_pinid!(PC10, PortC, 10);
def_pinid!(PC11, PortC, 11);
def_pinid!(PC12, PortC, 12);
def_pinid!(PC13, PortC, 13);
def_pinid!(PC14, PortC, 14);
def_pinid!(PC15, PortC, 15);

def_pinid!(PD00, PortD, 0);
def_pinid!(PD01, PortD, 1);
def_pinid!(PD02, PortD, 2);
def_pinid!(PD03, PortD, 3);
def_pinid!(PD04, PortD, 4);
def_pinid!(PD05, PortD, 5);
def_pinid!(PD06, PortD, 6);
def_pinid!(PD07, PortD, 7);
def_pinid!(PD08, PortD, 8);
def_pinid!(PD09, PortD, 9);
def_pinid!(PD10, PortD, 10);
def_pinid!(PD11, PortD, 11);
def_pinid!(PD12, PortD, 12);
def_pinid!(PD13, PortD, 13);
def_pinid!(PD14, PortD, 14);
def_pinid!(PD15, PortD, 15);

def_pinid!(PE00, PortE, 0);
def_pinid!(PE01, PortE, 1);
def_pinid!(PE02, PortE, 2);
def_pinid!(PE03, PortE, 3);
def_pinid!(PE04, PortE, 4);
def_pinid!(PE05, PortE, 5);
def_pinid!(PE06, PortE, 6);
def_pinid!(PE07, PortE, 7);
def_pinid!(PE08, PortE, 8);
def_pinid!(PE09, PortE, 9);
def_pinid!(PE10, PortE, 10);
def_pinid!(PE11, PortE, 11);
def_pinid!(PE12, PortE, 12);
def_pinid!(PE13, PortE, 13);
def_pinid!(PE14, PortE, 14);
def_pinid!(PE15, PortE, 15);

pub trait PinMode {
    const AF_NUM: u32;

    /// Set alternate function selection register
    unsafe fn set_afsel<I: PinId>(regs: &Iox) {
        unsafe {
            if I::INDEX < 8 {
                // index 0 to 7 -> use AFSELxL
                let shift = 2 * I::INDEX;
                I::Port::afsel_l(&regs).clear_bits(|w| w.bits(!(0b11 << shift)));
                I::Port::afsel_l(&regs).set_bits(|w| w.bits(Self::AF_NUM << shift));
            } else {
                // index 8 to 15 -> use AFSELxH
                let shift = 2 * (I::INDEX - 8);
                I::Port::afsel_h(&regs).clear_bits(|w| w.bits(!(0b11 << shift)));
                I::Port::afsel_h(&regs).set_bits(|w| w.bits(Self::AF_NUM << shift));
            }
        }
    }

    /// Set output enable register
    unsafe fn set_oe<I: PinId>(regs: &Iox) {
        unsafe {
            // Disable output
            I::Port::gpio_oe(regs).clear_bits(|w| w.bits(!(1 << I::INDEX)));
        }
    }
}

pub struct Output {}

impl PinMode for Output {
    const AF_NUM: u32 = 0;

    unsafe fn set_oe<I: PinId>(regs: &Iox) {
        unsafe {
            // Enable output
            I::Port::gpio_oe(regs).set_bits(|w| w.bits(1 << I::INDEX));
        }
    }
}

pub struct Input {}

impl PinMode for Input {
    const AF_NUM: u32 = 0;
}

pub struct Alternate<const AF_NUM: u32> {}

impl<const ALT_AF_NUM: u32> PinMode for Alternate<ALT_AF_NUM> {
    const AF_NUM: u32 = ALT_AF_NUM;
}

pub struct Pin<I: PinId, M: PinMode> {
    id: PhantomData<I>,
    mode: PhantomData<M>,
}

impl<I: PinId, M: PinMode> Pin<I, M> {
    pub(crate) fn new() -> Self {
        Self {
            id: PhantomData,
            mode: PhantomData,
        }
    }

    pub fn into_mode<M1: PinMode>(self) -> Pin<I, M1> {
        // SAFETY:
        //   - `Pin` cannot be created without an `Iox` instance.
        //   - There is only one `Pin` instance that touches the same bits.
        //   - Race conditions are avoided by atomic operations.
        unsafe {
            let regs = Iox::steal();

            M1::set_afsel::<I>(&regs);
            M1::set_oe::<I>(&regs);
        }

        Pin {
            id: PhantomData,
            mode: PhantomData,
        }
    }

    pub fn into_output(self) -> Pin<I, Output> {
        self.into_mode()
    }

    pub fn into_input(self) -> Pin<I, Output> {
        self.into_mode()
    }
}

impl<I: PinId> embedded_hal::digital::ErrorType for Pin<I, Output> {
    type Error = Infallible;
}

impl<I: PinId> embedded_hal::digital::OutputPin for Pin<I, Output> {
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        // SAFETY:
        //   - `Pin` cannot be created without an `Iox` instance.
        //   - There is only one `Pin` instance that touches the same bits.
        //   - Race conditions are avoided by atomic operations.
        unsafe {
            let regs = Iox::steal();
            I::Port::gpio_out(&regs).clear_bits(|w| w.bits(!(1 << I::INDEX)));
        }

        Ok(())
    }

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        // SAFETY:
        //   - `Pin` cannot be created without an `Iox` instance.
        //   - There is only one `Pin` instance that touches the same bits.
        //   - Race conditions are avoided by atomic operations.
        unsafe {
            let regs = Iox::steal();
            I::Port::gpio_out(&regs).set_bits(|w| w.bits(1 << I::INDEX));
        }

        Ok(())
    }
}

impl<I: PinId> embedded_hal::digital::StatefulOutputPin for Pin<I, Output> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        // SAFETY:
        //   - `Pin` cannot be created without an `Iox` instance.
        //   - There is only one `Pin` instance that touches the same bits.
        unsafe {
            let regs = Iox::steal();
            Ok(I::Port::gpio_out(&regs).read().bits() & (1 << I::INDEX) != 0)
        }
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        // SAFETY:
        //   - `Pin` cannot be created without an `Iox` instance.
        //   - There is only one `Pin` instance that touches the same bits.
        unsafe {
            let regs = Iox::steal();
            Ok(I::Port::gpio_out(&regs).read().bits() & (1 << I::INDEX) == 0)
        }
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        // SAFETY:
        //   - `Pin` cannot be created without an `Iox` instance.
        //   - There is only one `Pin` instance that touches the same bits.
        //   - Race conditions are avoided by atomic operations.
        unsafe {
            let regs = Iox::steal();
            I::Port::gpio_out(&regs).toggle_bits(|w| w.bits(1 << I::INDEX));
        }

        Ok(())
    }
}

impl<I: PinId> embedded_hal::digital::ErrorType for Pin<I, Input> {
    type Error = Infallible;
}

impl<I: PinId> embedded_hal::digital::InputPin for Pin<I, Input> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        // SAFETY:
        //   - `Pin` cannot be created without an `Iox` instance.
        //   - There is only one `Pin` instance that touches the same bits.
        unsafe {
            let regs = Iox::steal();
            Ok(I::Port::gpio_in(&regs).read().bits() & (1 << I::INDEX) != 0)
        }
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        // SAFETY:
        //   - `Pin` cannot be created without an `Iox` instance.
        //   - There is only one `Pin` instance that touches the same bits.
        unsafe {
            let regs = Iox::steal();
            Ok(I::Port::gpio_in(&regs).read().bits() & (1 << I::INDEX) == 0)
        }
    }
}
