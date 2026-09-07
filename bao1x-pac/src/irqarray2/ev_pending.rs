#[doc = "Register `EV_PENDING` reader"]
pub type R = crate::R<EvPendingSpec>;
#[doc = "Register `EV_PENDING` writer"]
pub type W = crate::W<EvPendingSpec>;
#[doc = "Field `qfcirq` reader - `1` when a \"qfcirq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type QfcirqR = crate::BitReader;
#[doc = "Field `qfcirq` writer - `1` when a \"qfcirq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type QfcirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mdmairq` reader - `1` when a \"mdmairq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MdmairqR = crate::BitReader;
#[doc = "Field `mdmairq` writer - `1` when a \"mdmairq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MdmairqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_available` reader - `1` when a \"mbox_irq_available\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqAvailableR = crate::BitReader;
#[doc = "Field `mbox_irq_available` writer - `1` when a \"mbox_irq_available\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqAvailableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_abort_init` reader - `1` when a \"mbox_irq_abort_init\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqAbortInitR = crate::BitReader;
#[doc = "Field `mbox_irq_abort_init` writer - `1` when a \"mbox_irq_abort_init\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqAbortInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_done` reader - `1` when a \"mbox_irq_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqDoneR = crate::BitReader;
#[doc = "Field `mbox_irq_done` writer - `1` when a \"mbox_irq_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_error` reader - `1` when a \"mbox_irq_error\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqErrorR = crate::BitReader;
#[doc = "Field `mbox_irq_error` writer - `1` when a \"mbox_irq_error\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type MboxIrqErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s6` reader - `1` when a \"nc_b2s6\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s6R = crate::BitReader;
#[doc = "Field `nc_b2s6` writer - `1` when a \"nc_b2s6\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s7` reader - `1` when a \"nc_b2s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s7R = crate::BitReader;
#[doc = "Field `nc_b2s7` writer - `1` when a \"nc_b2s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s8` reader - `1` when a \"nc_b2s8\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s8R = crate::BitReader;
#[doc = "Field `nc_b2s8` writer - `1` when a \"nc_b2s8\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s9` reader - `1` when a \"nc_b2s9\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s9R = crate::BitReader;
#[doc = "Field `nc_b2s9` writer - `1` when a \"nc_b2s9\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s10` reader - `1` when a \"nc_b2s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s10R = crate::BitReader;
#[doc = "Field `nc_b2s10` writer - `1` when a \"nc_b2s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s11` reader - `1` when a \"nc_b2s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s11R = crate::BitReader;
#[doc = "Field `nc_b2s11` writer - `1` when a \"nc_b2s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s12` reader - `1` when a \"nc_b2s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s12R = crate::BitReader;
#[doc = "Field `nc_b2s12` writer - `1` when a \"nc_b2s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s13` reader - `1` when a \"nc_b2s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s13R = crate::BitReader;
#[doc = "Field `nc_b2s13` writer - `1` when a \"nc_b2s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s14` reader - `1` when a \"nc_b2s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s14R = crate::BitReader;
#[doc = "Field `nc_b2s14` writer - `1` when a \"nc_b2s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type NcB2s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aowkupint` reader - `1` when a \"aowkupint\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type AowkupintR = crate::BitReader;
#[doc = "Field `aowkupint` writer - `1` when a \"aowkupint\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
pub type AowkupintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - `1` when a \"qfcirq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn qfcirq(&self) -> QfcirqR {
        QfcirqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - `1` when a \"mdmairq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mdmairq(&self) -> MdmairqR {
        MdmairqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - `1` when a \"mbox_irq_available\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_available(&self) -> MboxIrqAvailableR {
        MboxIrqAvailableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - `1` when a \"mbox_irq_abort_init\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_abort_init(&self) -> MboxIrqAbortInitR {
        MboxIrqAbortInitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - `1` when a \"mbox_irq_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_done(&self) -> MboxIrqDoneR {
        MboxIrqDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - `1` when a \"mbox_irq_error\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_error(&self) -> MboxIrqErrorR {
        MboxIrqErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - `1` when a \"nc_b2s6\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s6(&self) -> NcB2s6R {
        NcB2s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - `1` when a \"nc_b2s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s7(&self) -> NcB2s7R {
        NcB2s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - `1` when a \"nc_b2s8\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s8(&self) -> NcB2s8R {
        NcB2s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - `1` when a \"nc_b2s9\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s9(&self) -> NcB2s9R {
        NcB2s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - `1` when a \"nc_b2s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s10(&self) -> NcB2s10R {
        NcB2s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - `1` when a \"nc_b2s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s11(&self) -> NcB2s11R {
        NcB2s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - `1` when a \"nc_b2s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s12(&self) -> NcB2s12R {
        NcB2s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - `1` when a \"nc_b2s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s13(&self) -> NcB2s13R {
        NcB2s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - `1` when a \"nc_b2s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s14(&self) -> NcB2s14R {
        NcB2s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - `1` when a \"aowkupint\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn aowkupint(&self) -> AowkupintR {
        AowkupintR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - `1` when a \"qfcirq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn qfcirq(&mut self) -> QfcirqW<'_, EvPendingSpec> {
        QfcirqW::new(self, 0)
    }
    #[doc = "Bit 1 - `1` when a \"mdmairq\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mdmairq(&mut self) -> MdmairqW<'_, EvPendingSpec> {
        MdmairqW::new(self, 1)
    }
    #[doc = "Bit 2 - `1` when a \"mbox_irq_available\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_available(&mut self) -> MboxIrqAvailableW<'_, EvPendingSpec> {
        MboxIrqAvailableW::new(self, 2)
    }
    #[doc = "Bit 3 - `1` when a \"mbox_irq_abort_init\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_abort_init(&mut self) -> MboxIrqAbortInitW<'_, EvPendingSpec> {
        MboxIrqAbortInitW::new(self, 3)
    }
    #[doc = "Bit 4 - `1` when a \"mbox_irq_done\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_done(&mut self) -> MboxIrqDoneW<'_, EvPendingSpec> {
        MboxIrqDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - `1` when a \"mbox_irq_error\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn mbox_irq_error(&mut self) -> MboxIrqErrorW<'_, EvPendingSpec> {
        MboxIrqErrorW::new(self, 5)
    }
    #[doc = "Bit 6 - `1` when a \"nc_b2s6\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s6(&mut self) -> NcB2s6W<'_, EvPendingSpec> {
        NcB2s6W::new(self, 6)
    }
    #[doc = "Bit 7 - `1` when a \"nc_b2s7\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s7(&mut self) -> NcB2s7W<'_, EvPendingSpec> {
        NcB2s7W::new(self, 7)
    }
    #[doc = "Bit 8 - `1` when a \"nc_b2s8\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s8(&mut self) -> NcB2s8W<'_, EvPendingSpec> {
        NcB2s8W::new(self, 8)
    }
    #[doc = "Bit 9 - `1` when a \"nc_b2s9\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s9(&mut self) -> NcB2s9W<'_, EvPendingSpec> {
        NcB2s9W::new(self, 9)
    }
    #[doc = "Bit 10 - `1` when a \"nc_b2s10\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s10(&mut self) -> NcB2s10W<'_, EvPendingSpec> {
        NcB2s10W::new(self, 10)
    }
    #[doc = "Bit 11 - `1` when a \"nc_b2s11\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s11(&mut self) -> NcB2s11W<'_, EvPendingSpec> {
        NcB2s11W::new(self, 11)
    }
    #[doc = "Bit 12 - `1` when a \"nc_b2s12\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s12(&mut self) -> NcB2s12W<'_, EvPendingSpec> {
        NcB2s12W::new(self, 12)
    }
    #[doc = "Bit 13 - `1` when a \"nc_b2s13\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s13(&mut self) -> NcB2s13W<'_, EvPendingSpec> {
        NcB2s13W::new(self, 13)
    }
    #[doc = "Bit 14 - `1` when a \"nc_b2s14\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn nc_b2s14(&mut self) -> NcB2s14W<'_, EvPendingSpec> {
        NcB2s14W::new(self, 14)
    }
    #[doc = "Bit 15 - `1` when a \"aowkupint\" event occurs. This event uses an `EventSourceFlex` form of triggering"]
    #[inline(always)]
    pub fn aowkupint(&mut self) -> AowkupintW<'_, EvPendingSpec> {
        AowkupintW::new(self, 15)
    }
}
#[doc = "`1` when a \"aowkupint\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_pending::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_pending::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
