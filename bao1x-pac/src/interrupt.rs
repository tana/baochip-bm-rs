pub use riscv::interrupt::Exception;
pub use riscv::interrupt::Interrupt as CoreInterrupt;
pub use riscv::{
    interrupt::{disable, enable, free, nested},
    ExceptionNumber, HartIdNumber, InterruptNumber, PriorityNumber,
};
pub type Trap = riscv::interrupt::Trap<CoreInterrupt, Exception>;
#[doc = r" Retrieves the cause of a trap in the current hart."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it returns an error."]
#[inline]
pub fn try_cause() -> riscv::result::Result<Trap> {
    riscv::interrupt::try_cause()
}
#[doc = r" Retrieves the cause of a trap in the current hart (machine mode)."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it panics."]
#[inline]
pub fn cause() -> Trap {
    try_cause().unwrap()
}
#[doc = r" External interrupts. These interrupts are handled by the external peripherals."]
# [riscv :: pac_enum (unsafe ExternalInterruptNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalInterrupt {
    #[doc = "0 - irqarray0"]
    irqarray0 = 0,
    #[doc = "1 - irqarray1"]
    irqarray1 = 1,
    #[doc = "2 - irqarray2"]
    irqarray2 = 2,
    #[doc = "3 - irqarray3"]
    irqarray3 = 3,
    #[doc = "4 - irqarray4"]
    irqarray4 = 4,
    #[doc = "5 - irqarray5"]
    irqarray5 = 5,
    #[doc = "6 - irqarray6"]
    irqarray6 = 6,
    #[doc = "7 - irqarray7"]
    irqarray7 = 7,
    #[doc = "8 - irqarray8"]
    irqarray8 = 8,
    #[doc = "9 - irqarray9"]
    irqarray9 = 9,
    #[doc = "10 - irqarray10"]
    irqarray10 = 10,
    #[doc = "11 - irqarray11"]
    irqarray11 = 11,
    #[doc = "12 - irqarray12"]
    irqarray12 = 12,
    #[doc = "13 - irqarray13"]
    irqarray13 = 13,
    #[doc = "14 - irqarray14"]
    irqarray14 = 14,
    #[doc = "15 - irqarray15"]
    irqarray15 = 15,
    #[doc = "16 - irqarray16"]
    irqarray16 = 16,
    #[doc = "17 - irqarray17"]
    irqarray17 = 17,
    #[doc = "18 - irqarray18"]
    irqarray18 = 18,
    #[doc = "19 - irqarray19"]
    irqarray19 = 19,
    #[doc = "20 - ticktimer"]
    ticktimer = 20,
    #[doc = "21 - susres"]
    susres = 21,
    #[doc = "22 - mailbox"]
    mailbox = 22,
    #[doc = "23 - mb_client"]
    mb_client = 23,
    #[doc = "30 - timer0"]
    timer0 = 30,
}
