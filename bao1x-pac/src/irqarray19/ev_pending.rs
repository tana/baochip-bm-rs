#[doc = "Register `EV_PENDING` reader"]
pub type R = crate::R<EvPendingSpec>;
#[doc = "Register `EV_PENDING` writer"]
pub type W = crate::W<EvPendingSpec>;
#[doc = "Field `mbox_irq_available_dupe` reader - `1` when a \"mbox_irq_available_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqAvailableDupeR = crate::BitReader;
#[doc = "Field `mbox_irq_available_dupe` writer - `1` when a \"mbox_irq_available_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqAvailableDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_abort_init_dupe` reader - `1` when a \"mbox_irq_abort_init_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqAbortInitDupeR = crate::BitReader;
#[doc = "Field `mbox_irq_abort_init_dupe` writer - `1` when a \"mbox_irq_abort_init_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqAbortInitDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_done_dupe` reader - `1` when a \"mbox_irq_done_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqDoneDupeR = crate::BitReader;
#[doc = "Field `mbox_irq_done_dupe` writer - `1` when a \"mbox_irq_done_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqDoneDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_error_dupe` reader - `1` when a \"mbox_irq_error_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqErrorDupeR = crate::BitReader;
#[doc = "Field `mbox_irq_error_dupe` writer - `1` when a \"mbox_irq_error_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqErrorDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq0_dupe` reader - `1` when a \"pioirq0_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq0DupeR = crate::BitReader;
#[doc = "Field `pioirq0_dupe` writer - `1` when a \"pioirq0_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq0DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq1_dupe` reader - `1` when a \"pioirq1_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq1DupeR = crate::BitReader;
#[doc = "Field `pioirq1_dupe` writer - `1` when a \"pioirq1_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq1DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq2_dupe` reader - `1` when a \"pioirq2_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq2DupeR = crate::BitReader;
#[doc = "Field `pioirq2_dupe` writer - `1` when a \"pioirq2_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq2DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pioirq3_dupe` reader - `1` when a \"pioirq3_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq3DupeR = crate::BitReader;
#[doc = "Field `pioirq3_dupe` writer - `1` when a \"pioirq3_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type Pioirq3DupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_rx_dupe` reader - `1` when a \"sdio_rx_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioRxDupeR = crate::BitReader;
#[doc = "Field `sdio_rx_dupe` writer - `1` when a \"sdio_rx_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioRxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_tx_dupe` reader - `1` when a \"sdio_tx_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioTxDupeR = crate::BitReader;
#[doc = "Field `sdio_tx_dupe` writer - `1` when a \"sdio_tx_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioTxDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_eot_dupe` reader - `1` when a \"sdio_eot_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioEotDupeR = crate::BitReader;
#[doc = "Field `sdio_eot_dupe` writer - `1` when a \"sdio_eot_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioEotDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sdio_err_dupe` reader - `1` when a \"sdio_err_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioErrDupeR = crate::BitReader;
#[doc = "Field `sdio_err_dupe` writer - `1` when a \"sdio_err_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type SdioErrDupeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b19s12` reader - `1` when a \"nc_b19s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB19s12R = crate::BitReader;
#[doc = "Field `nc_b19s12` writer - `1` when a \"nc_b19s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB19s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b19s13` reader - `1` when a \"nc_b19s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB19s13R = crate::BitReader;
#[doc = "Field `nc_b19s13` writer - `1` when a \"nc_b19s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB19s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b19s14` reader - `1` when a \"nc_b19s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB19s14R = crate::BitReader;
#[doc = "Field `nc_b19s14` writer - `1` when a \"nc_b19s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB19s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b19s15` reader - `1` when a \"nc_b19s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB19s15R = crate::BitReader;
#[doc = "Field `nc_b19s15` writer - `1` when a \"nc_b19s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB19s15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - `1` when a \"mbox_irq_available_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_available_dupe(&self) -> MboxIrqAvailableDupeR {
        MboxIrqAvailableDupeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - `1` when a \"mbox_irq_abort_init_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_abort_init_dupe(&self) -> MboxIrqAbortInitDupeR {
        MboxIrqAbortInitDupeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - `1` when a \"mbox_irq_done_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_done_dupe(&self) -> MboxIrqDoneDupeR {
        MboxIrqDoneDupeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - `1` when a \"mbox_irq_error_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_error_dupe(&self) -> MboxIrqErrorDupeR {
        MboxIrqErrorDupeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - `1` when a \"pioirq0_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq0_dupe(&self) -> Pioirq0DupeR {
        Pioirq0DupeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - `1` when a \"pioirq1_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq1_dupe(&self) -> Pioirq1DupeR {
        Pioirq1DupeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - `1` when a \"pioirq2_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq2_dupe(&self) -> Pioirq2DupeR {
        Pioirq2DupeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - `1` when a \"pioirq3_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq3_dupe(&self) -> Pioirq3DupeR {
        Pioirq3DupeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - `1` when a \"sdio_rx_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_rx_dupe(&self) -> SdioRxDupeR {
        SdioRxDupeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - `1` when a \"sdio_tx_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_tx_dupe(&self) -> SdioTxDupeR {
        SdioTxDupeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - `1` when a \"sdio_eot_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_eot_dupe(&self) -> SdioEotDupeR {
        SdioEotDupeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - `1` when a \"sdio_err_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_err_dupe(&self) -> SdioErrDupeR {
        SdioErrDupeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - `1` when a \"nc_b19s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b19s12(&self) -> NcB19s12R {
        NcB19s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - `1` when a \"nc_b19s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b19s13(&self) -> NcB19s13R {
        NcB19s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - `1` when a \"nc_b19s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b19s14(&self) -> NcB19s14R {
        NcB19s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - `1` when a \"nc_b19s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b19s15(&self) -> NcB19s15R {
        NcB19s15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - `1` when a \"mbox_irq_available_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_available_dupe(&mut self) -> MboxIrqAvailableDupeW<'_, EvPendingSpec> {
        MboxIrqAvailableDupeW::new(self, 0)
    }
    #[doc = "Bit 1 - `1` when a \"mbox_irq_abort_init_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_abort_init_dupe(&mut self) -> MboxIrqAbortInitDupeW<'_, EvPendingSpec> {
        MboxIrqAbortInitDupeW::new(self, 1)
    }
    #[doc = "Bit 2 - `1` when a \"mbox_irq_done_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_done_dupe(&mut self) -> MboxIrqDoneDupeW<'_, EvPendingSpec> {
        MboxIrqDoneDupeW::new(self, 2)
    }
    #[doc = "Bit 3 - `1` when a \"mbox_irq_error_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_error_dupe(&mut self) -> MboxIrqErrorDupeW<'_, EvPendingSpec> {
        MboxIrqErrorDupeW::new(self, 3)
    }
    #[doc = "Bit 4 - `1` when a \"pioirq0_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq0_dupe(&mut self) -> Pioirq0DupeW<'_, EvPendingSpec> {
        Pioirq0DupeW::new(self, 4)
    }
    #[doc = "Bit 5 - `1` when a \"pioirq1_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq1_dupe(&mut self) -> Pioirq1DupeW<'_, EvPendingSpec> {
        Pioirq1DupeW::new(self, 5)
    }
    #[doc = "Bit 6 - `1` when a \"pioirq2_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq2_dupe(&mut self) -> Pioirq2DupeW<'_, EvPendingSpec> {
        Pioirq2DupeW::new(self, 6)
    }
    #[doc = "Bit 7 - `1` when a \"pioirq3_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn pioirq3_dupe(&mut self) -> Pioirq3DupeW<'_, EvPendingSpec> {
        Pioirq3DupeW::new(self, 7)
    }
    #[doc = "Bit 8 - `1` when a \"sdio_rx_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_rx_dupe(&mut self) -> SdioRxDupeW<'_, EvPendingSpec> {
        SdioRxDupeW::new(self, 8)
    }
    #[doc = "Bit 9 - `1` when a \"sdio_tx_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_tx_dupe(&mut self) -> SdioTxDupeW<'_, EvPendingSpec> {
        SdioTxDupeW::new(self, 9)
    }
    #[doc = "Bit 10 - `1` when a \"sdio_eot_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_eot_dupe(&mut self) -> SdioEotDupeW<'_, EvPendingSpec> {
        SdioEotDupeW::new(self, 10)
    }
    #[doc = "Bit 11 - `1` when a \"sdio_err_dupe\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn sdio_err_dupe(&mut self) -> SdioErrDupeW<'_, EvPendingSpec> {
        SdioErrDupeW::new(self, 11)
    }
    #[doc = "Bit 12 - `1` when a \"nc_b19s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b19s12(&mut self) -> NcB19s12W<'_, EvPendingSpec> {
        NcB19s12W::new(self, 12)
    }
    #[doc = "Bit 13 - `1` when a \"nc_b19s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b19s13(&mut self) -> NcB19s13W<'_, EvPendingSpec> {
        NcB19s13W::new(self, 13)
    }
    #[doc = "Bit 14 - `1` when a \"nc_b19s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b19s14(&mut self) -> NcB19s14W<'_, EvPendingSpec> {
        NcB19s14W::new(self, 14)
    }
    #[doc = "Bit 15 - `1` when a \"nc_b19s15\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b19s15(&mut self) -> NcB19s15W<'_, EvPendingSpec> {
        NcB19s15W::new(self, 15)
    }
}
#[doc = "`1` when a \"nc_b19s15\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvPendingSpec;
impl crate::RegisterSpec for EvPendingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_pending::R`](R) reader structure"]
impl crate::Readable for EvPendingSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_pending::W`](W) writer structure"]
impl crate::Writable for EvPendingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_PENDING to value 0"]
impl crate::Resettable for EvPendingSpec {}
