#[doc = "Register `SFR_STATUS` reader"]
pub type R = crate::R<SfrStatusSpec>;
#[doc = "Register `SFR_STATUS` writer"]
pub type W = crate::W<SfrStatusSpec>;
#[doc = "Field `rx_avail` reader - rx_avail read only status register"]
pub type RxAvailR = crate::BitReader;
#[doc = "Field `rx_avail` writer - rx_avail read only status register"]
pub type RxAvailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_free` reader - tx_free read only status register"]
pub type TxFreeR = crate::BitReader;
#[doc = "Field `tx_free` writer - tx_free read only status register"]
pub type TxFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_in_progress` reader - abort_in_progress read only status register"]
pub type AbortInProgressR = crate::BitReader;
#[doc = "Field `abort_in_progress` writer - abort_in_progress read only status register"]
pub type AbortInProgressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_ack` reader - abort_ack read only status register"]
pub type AbortAckR = crate::BitReader;
#[doc = "Field `abort_ack` writer - abort_ack read only status register"]
pub type AbortAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_err` reader - tx_err read only status register"]
pub type TxErrR = crate::BitReader;
#[doc = "Field `tx_err` writer - tx_err read only status register"]
pub type TxErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_err` reader - rx_err read only status register"]
pub type RxErrR = crate::BitReader;
#[doc = "Field `rx_err` writer - rx_err read only status register"]
pub type RxErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - rx_avail read only status register"]
    #[inline(always)]
    pub fn rx_avail(&self) -> RxAvailR {
        RxAvailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - tx_free read only status register"]
    #[inline(always)]
    pub fn tx_free(&self) -> TxFreeR {
        TxFreeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - abort_in_progress read only status register"]
    #[inline(always)]
    pub fn abort_in_progress(&self) -> AbortInProgressR {
        AbortInProgressR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - abort_ack read only status register"]
    #[inline(always)]
    pub fn abort_ack(&self) -> AbortAckR {
        AbortAckR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - tx_err read only status register"]
    #[inline(always)]
    pub fn tx_err(&self) -> TxErrR {
        TxErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - rx_err read only status register"]
    #[inline(always)]
    pub fn rx_err(&self) -> RxErrR {
        RxErrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - rx_avail read only status register"]
    #[inline(always)]
    pub fn rx_avail(&mut self) -> RxAvailW<'_, SfrStatusSpec> {
        RxAvailW::new(self, 0)
    }
    #[doc = "Bit 1 - tx_free read only status register"]
    #[inline(always)]
    pub fn tx_free(&mut self) -> TxFreeW<'_, SfrStatusSpec> {
        TxFreeW::new(self, 1)
    }
    #[doc = "Bit 2 - abort_in_progress read only status register"]
    #[inline(always)]
    pub fn abort_in_progress(&mut self) -> AbortInProgressW<'_, SfrStatusSpec> {
        AbortInProgressW::new(self, 2)
    }
    #[doc = "Bit 3 - abort_ack read only status register"]
    #[inline(always)]
    pub fn abort_ack(&mut self) -> AbortAckW<'_, SfrStatusSpec> {
        AbortAckW::new(self, 3)
    }
    #[doc = "Bit 4 - tx_err read only status register"]
    #[inline(always)]
    pub fn tx_err(&mut self) -> TxErrW<'_, SfrStatusSpec> {
        TxErrW::new(self, 4)
    }
    #[doc = "Bit 5 - rx_err read only status register"]
    #[inline(always)]
    pub fn rx_err(&mut self) -> RxErrW<'_, SfrStatusSpec> {
        RxErrW::new(self, 5)
    }
}
#[doc = "See `mbox_v0.1.sv#L107 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/vexriscv/lib/mbox_v0.1.sv#L107>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrStatusSpec;
impl crate::RegisterSpec for SfrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_status::R`](R) reader structure"]
impl crate::Readable for SfrStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_status::W`](W) writer structure"]
impl crate::Writable for SfrStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_STATUS to value 0"]
impl crate::Resettable for SfrStatusSpec {}
