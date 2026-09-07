#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `mbox_irq_available_dupe` reader - Level of the ``mbox_irq_available_dupe`` event"]
pub type MboxIrqAvailableDupeR = crate::BitReader;
#[doc = "Field `mbox_irq_available_dupe` writer - Level of the ``mbox_irq_available_dupe`` event"]
pub type MboxIrqAvailableDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_abort_init_dupe` reader - Level of the ``mbox_irq_abort_init_dupe`` event"]
pub type MboxIrqAbortInitDupeR = crate::BitReader;
#[doc = "Field `mbox_irq_abort_init_dupe` writer - Level of the ``mbox_irq_abort_init_dupe`` event"]
pub type MboxIrqAbortInitDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_done_dupe` reader - Level of the ``mbox_irq_done_dupe`` event"]
pub type MboxIrqDoneDupeR = crate::BitReader;
#[doc = "Field `mbox_irq_done_dupe` writer - Level of the ``mbox_irq_done_dupe`` event"]
pub type MboxIrqDoneDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_error_dupe` reader - Level of the ``mbox_irq_error_dupe`` event"]
pub type MboxIrqErrorDupeR = crate::BitReader;
#[doc = "Field `mbox_irq_error_dupe` writer - Level of the ``mbox_irq_error_dupe`` event"]
pub type MboxIrqErrorDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq0_dupe` reader - Level of the ``pioirq0_dupe`` event"]
pub type Pioirq0DupeR = crate::BitReader;
#[doc = "Field `pioirq0_dupe` writer - Level of the ``pioirq0_dupe`` event"]
pub type Pioirq0DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq1_dupe` reader - Level of the ``pioirq1_dupe`` event"]
pub type Pioirq1DupeR = crate::BitReader;
#[doc = "Field `pioirq1_dupe` writer - Level of the ``pioirq1_dupe`` event"]
pub type Pioirq1DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq2_dupe` reader - Level of the ``pioirq2_dupe`` event"]
pub type Pioirq2DupeR = crate::BitReader;
#[doc = "Field `pioirq2_dupe` writer - Level of the ``pioirq2_dupe`` event"]
pub type Pioirq2DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq3_dupe` reader - Level of the ``pioirq3_dupe`` event"]
pub type Pioirq3DupeR = crate::BitReader;
#[doc = "Field `pioirq3_dupe` writer - Level of the ``pioirq3_dupe`` event"]
pub type Pioirq3DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_rx_dupe` reader - Level of the ``sdio_rx_dupe`` event"]
pub type SdioRxDupeR = crate::BitReader;
#[doc = "Field `sdio_rx_dupe` writer - Level of the ``sdio_rx_dupe`` event"]
pub type SdioRxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_tx_dupe` reader - Level of the ``sdio_tx_dupe`` event"]
pub type SdioTxDupeR = crate::BitReader;
#[doc = "Field `sdio_tx_dupe` writer - Level of the ``sdio_tx_dupe`` event"]
pub type SdioTxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_eot_dupe` reader - Level of the ``sdio_eot_dupe`` event"]
pub type SdioEotDupeR = crate::BitReader;
#[doc = "Field `sdio_eot_dupe` writer - Level of the ``sdio_eot_dupe`` event"]
pub type SdioEotDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_err_dupe` reader - Level of the ``sdio_err_dupe`` event"]
pub type SdioErrDupeR = crate::BitReader;
#[doc = "Field `sdio_err_dupe` writer - Level of the ``sdio_err_dupe`` event"]
pub type SdioErrDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b19s12` reader - Level of the ``nc_b19s12`` event"]
pub type NcB19s12R = crate::BitReader;
#[doc = "Field `nc_b19s12` writer - Level of the ``nc_b19s12`` event"]
pub type NcB19s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b19s13` reader - Level of the ``nc_b19s13`` event"]
pub type NcB19s13R = crate::BitReader;
#[doc = "Field `nc_b19s13` writer - Level of the ``nc_b19s13`` event"]
pub type NcB19s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b19s14` reader - Level of the ``nc_b19s14`` event"]
pub type NcB19s14R = crate::BitReader;
#[doc = "Field `nc_b19s14` writer - Level of the ``nc_b19s14`` event"]
pub type NcB19s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b19s15` reader - Level of the ``nc_b19s15`` event"]
pub type NcB19s15R = crate::BitReader;
#[doc = "Field `nc_b19s15` writer - Level of the ``nc_b19s15`` event"]
pub type NcB19s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``mbox_irq_available_dupe`` event"]
    #[inline(always)]
    pub fn mbox_irq_available_dupe(&self) -> MboxIrqAvailableDupeR {
        MboxIrqAvailableDupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``mbox_irq_abort_init_dupe`` event"]
    #[inline(always)]
    pub fn mbox_irq_abort_init_dupe(&self) -> MboxIrqAbortInitDupeR {
        MboxIrqAbortInitDupeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``mbox_irq_done_dupe`` event"]
    #[inline(always)]
    pub fn mbox_irq_done_dupe(&self) -> MboxIrqDoneDupeR {
        MboxIrqDoneDupeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``mbox_irq_error_dupe`` event"]
    #[inline(always)]
    pub fn mbox_irq_error_dupe(&self) -> MboxIrqErrorDupeR {
        MboxIrqErrorDupeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``pioirq0_dupe`` event"]
    #[inline(always)]
    pub fn pioirq0_dupe(&self) -> Pioirq0DupeR {
        Pioirq0DupeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``pioirq1_dupe`` event"]
    #[inline(always)]
    pub fn pioirq1_dupe(&self) -> Pioirq1DupeR {
        Pioirq1DupeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``pioirq2_dupe`` event"]
    #[inline(always)]
    pub fn pioirq2_dupe(&self) -> Pioirq2DupeR {
        Pioirq2DupeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``pioirq3_dupe`` event"]
    #[inline(always)]
    pub fn pioirq3_dupe(&self) -> Pioirq3DupeR {
        Pioirq3DupeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``sdio_rx_dupe`` event"]
    #[inline(always)]
    pub fn sdio_rx_dupe(&self) -> SdioRxDupeR {
        SdioRxDupeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``sdio_tx_dupe`` event"]
    #[inline(always)]
    pub fn sdio_tx_dupe(&self) -> SdioTxDupeR {
        SdioTxDupeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``sdio_eot_dupe`` event"]
    #[inline(always)]
    pub fn sdio_eot_dupe(&self) -> SdioEotDupeR {
        SdioEotDupeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``sdio_err_dupe`` event"]
    #[inline(always)]
    pub fn sdio_err_dupe(&self) -> SdioErrDupeR {
        SdioErrDupeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``nc_b19s12`` event"]
    #[inline(always)]
    pub fn nc_b19s12(&self) -> NcB19s12R {
        NcB19s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``nc_b19s13`` event"]
    #[inline(always)]
    pub fn nc_b19s13(&self) -> NcB19s13R {
        NcB19s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``nc_b19s14`` event"]
    #[inline(always)]
    pub fn nc_b19s14(&self) -> NcB19s14R {
        NcB19s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``nc_b19s15`` event"]
    #[inline(always)]
    pub fn nc_b19s15(&self) -> NcB19s15R {
        NcB19s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``mbox_irq_available_dupe`` event"]
    #[inline(always)]
    pub fn mbox_irq_available_dupe(&mut self) -> MboxIrqAvailableDupeW<'_, EvStatusSpec> {
        MboxIrqAvailableDupeW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``mbox_irq_abort_init_dupe`` event"]
    #[inline(always)]
    pub fn mbox_irq_abort_init_dupe(&mut self) -> MboxIrqAbortInitDupeW<'_, EvStatusSpec> {
        MboxIrqAbortInitDupeW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``mbox_irq_done_dupe`` event"]
    #[inline(always)]
    pub fn mbox_irq_done_dupe(&mut self) -> MboxIrqDoneDupeW<'_, EvStatusSpec> {
        MboxIrqDoneDupeW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``mbox_irq_error_dupe`` event"]
    #[inline(always)]
    pub fn mbox_irq_error_dupe(&mut self) -> MboxIrqErrorDupeW<'_, EvStatusSpec> {
        MboxIrqErrorDupeW::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``pioirq0_dupe`` event"]
    #[inline(always)]
    pub fn pioirq0_dupe(&mut self) -> Pioirq0DupeW<'_, EvStatusSpec> {
        Pioirq0DupeW::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``pioirq1_dupe`` event"]
    #[inline(always)]
    pub fn pioirq1_dupe(&mut self) -> Pioirq1DupeW<'_, EvStatusSpec> {
        Pioirq1DupeW::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``pioirq2_dupe`` event"]
    #[inline(always)]
    pub fn pioirq2_dupe(&mut self) -> Pioirq2DupeW<'_, EvStatusSpec> {
        Pioirq2DupeW::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``pioirq3_dupe`` event"]
    #[inline(always)]
    pub fn pioirq3_dupe(&mut self) -> Pioirq3DupeW<'_, EvStatusSpec> {
        Pioirq3DupeW::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``sdio_rx_dupe`` event"]
    #[inline(always)]
    pub fn sdio_rx_dupe(&mut self) -> SdioRxDupeW<'_, EvStatusSpec> {
        SdioRxDupeW::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``sdio_tx_dupe`` event"]
    #[inline(always)]
    pub fn sdio_tx_dupe(&mut self) -> SdioTxDupeW<'_, EvStatusSpec> {
        SdioTxDupeW::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``sdio_eot_dupe`` event"]
    #[inline(always)]
    pub fn sdio_eot_dupe(&mut self) -> SdioEotDupeW<'_, EvStatusSpec> {
        SdioEotDupeW::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``sdio_err_dupe`` event"]
    #[inline(always)]
    pub fn sdio_err_dupe(&mut self) -> SdioErrDupeW<'_, EvStatusSpec> {
        SdioErrDupeW::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``nc_b19s12`` event"]
    #[inline(always)]
    pub fn nc_b19s12(&mut self) -> NcB19s12W<'_, EvStatusSpec> {
        NcB19s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``nc_b19s13`` event"]
    #[inline(always)]
    pub fn nc_b19s13(&mut self) -> NcB19s13W<'_, EvStatusSpec> {
        NcB19s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``nc_b19s14`` event"]
    #[inline(always)]
    pub fn nc_b19s14(&mut self) -> NcB19s14W<'_, EvStatusSpec> {
        NcB19s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``nc_b19s15`` event"]
    #[inline(always)]
    pub fn nc_b19s15(&mut self) -> NcB19s15W<'_, EvStatusSpec> {
        NcB19s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b19s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvStatusSpec;
impl crate::RegisterSpec for EvStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_status::R`](R) reader structure"]
impl crate::Readable for EvStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_status::W`](W) writer structure"]
impl crate::Writable for EvStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_STATUS to value 0"]
impl crate::Resettable for EvStatusSpec {}
