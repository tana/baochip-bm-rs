#[doc = "Register `EV_STATUS` reader"]
pub type R = crate::R<EvStatusSpec>;
#[doc = "Register `EV_STATUS` writer"]
pub type W = crate::W<EvStatusSpec>;
#[doc = "Field `qfcirq` reader - Level of the ``qfcirq`` event"]
pub type QfcirqR = crate::BitReader;
#[doc = "Field `qfcirq` writer - Level of the ``qfcirq`` event"]
pub type QfcirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mdmairq` reader - Level of the ``mdmairq`` event"]
pub type MdmairqR = crate::BitReader;
#[doc = "Field `mdmairq` writer - Level of the ``mdmairq`` event"]
pub type MdmairqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_available` reader - Level of the ``mbox_irq_available`` event"]
pub type MboxIrqAvailableR = crate::BitReader;
#[doc = "Field `mbox_irq_available` writer - Level of the ``mbox_irq_available`` event"]
pub type MboxIrqAvailableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_abort_init` reader - Level of the ``mbox_irq_abort_init`` event"]
pub type MboxIrqAbortInitR = crate::BitReader;
#[doc = "Field `mbox_irq_abort_init` writer - Level of the ``mbox_irq_abort_init`` event"]
pub type MboxIrqAbortInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_done` reader - Level of the ``mbox_irq_done`` event"]
pub type MboxIrqDoneR = crate::BitReader;
#[doc = "Field `mbox_irq_done` writer - Level of the ``mbox_irq_done`` event"]
pub type MboxIrqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_error` reader - Level of the ``mbox_irq_error`` event"]
pub type MboxIrqErrorR = crate::BitReader;
#[doc = "Field `mbox_irq_error` writer - Level of the ``mbox_irq_error`` event"]
pub type MboxIrqErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s6` reader - Level of the ``nc_b2s6`` event"]
pub type NcB2s6R = crate::BitReader;
#[doc = "Field `nc_b2s6` writer - Level of the ``nc_b2s6`` event"]
pub type NcB2s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s7` reader - Level of the ``nc_b2s7`` event"]
pub type NcB2s7R = crate::BitReader;
#[doc = "Field `nc_b2s7` writer - Level of the ``nc_b2s7`` event"]
pub type NcB2s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s8` reader - Level of the ``nc_b2s8`` event"]
pub type NcB2s8R = crate::BitReader;
#[doc = "Field `nc_b2s8` writer - Level of the ``nc_b2s8`` event"]
pub type NcB2s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s9` reader - Level of the ``nc_b2s9`` event"]
pub type NcB2s9R = crate::BitReader;
#[doc = "Field `nc_b2s9` writer - Level of the ``nc_b2s9`` event"]
pub type NcB2s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s10` reader - Level of the ``nc_b2s10`` event"]
pub type NcB2s10R = crate::BitReader;
#[doc = "Field `nc_b2s10` writer - Level of the ``nc_b2s10`` event"]
pub type NcB2s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s11` reader - Level of the ``nc_b2s11`` event"]
pub type NcB2s11R = crate::BitReader;
#[doc = "Field `nc_b2s11` writer - Level of the ``nc_b2s11`` event"]
pub type NcB2s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s12` reader - Level of the ``nc_b2s12`` event"]
pub type NcB2s12R = crate::BitReader;
#[doc = "Field `nc_b2s12` writer - Level of the ``nc_b2s12`` event"]
pub type NcB2s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s13` reader - Level of the ``nc_b2s13`` event"]
pub type NcB2s13R = crate::BitReader;
#[doc = "Field `nc_b2s13` writer - Level of the ``nc_b2s13`` event"]
pub type NcB2s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s14` reader - Level of the ``nc_b2s14`` event"]
pub type NcB2s14R = crate::BitReader;
#[doc = "Field `nc_b2s14` writer - Level of the ``nc_b2s14`` event"]
pub type NcB2s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aowkupint` reader - Level of the ``aowkupint`` event"]
pub type AowkupintR = crate::BitReader;
#[doc = "Field `aowkupint` writer - Level of the ``aowkupint`` event"]
pub type AowkupintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Level of the ``qfcirq`` event"]
    #[inline(always)]
    pub fn qfcirq(&self) -> QfcirqR {
        QfcirqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level of the ``mdmairq`` event"]
    #[inline(always)]
    pub fn mdmairq(&self) -> MdmairqR {
        MdmairqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level of the ``mbox_irq_available`` event"]
    #[inline(always)]
    pub fn mbox_irq_available(&self) -> MboxIrqAvailableR {
        MboxIrqAvailableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level of the ``mbox_irq_abort_init`` event"]
    #[inline(always)]
    pub fn mbox_irq_abort_init(&self) -> MboxIrqAbortInitR {
        MboxIrqAbortInitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level of the ``mbox_irq_done`` event"]
    #[inline(always)]
    pub fn mbox_irq_done(&self) -> MboxIrqDoneR {
        MboxIrqDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level of the ``mbox_irq_error`` event"]
    #[inline(always)]
    pub fn mbox_irq_error(&self) -> MboxIrqErrorR {
        MboxIrqErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level of the ``nc_b2s6`` event"]
    #[inline(always)]
    pub fn nc_b2s6(&self) -> NcB2s6R {
        NcB2s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level of the ``nc_b2s7`` event"]
    #[inline(always)]
    pub fn nc_b2s7(&self) -> NcB2s7R {
        NcB2s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level of the ``nc_b2s8`` event"]
    #[inline(always)]
    pub fn nc_b2s8(&self) -> NcB2s8R {
        NcB2s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level of the ``nc_b2s9`` event"]
    #[inline(always)]
    pub fn nc_b2s9(&self) -> NcB2s9R {
        NcB2s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level of the ``nc_b2s10`` event"]
    #[inline(always)]
    pub fn nc_b2s10(&self) -> NcB2s10R {
        NcB2s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level of the ``nc_b2s11`` event"]
    #[inline(always)]
    pub fn nc_b2s11(&self) -> NcB2s11R {
        NcB2s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level of the ``nc_b2s12`` event"]
    #[inline(always)]
    pub fn nc_b2s12(&self) -> NcB2s12R {
        NcB2s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level of the ``nc_b2s13`` event"]
    #[inline(always)]
    pub fn nc_b2s13(&self) -> NcB2s13R {
        NcB2s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level of the ``nc_b2s14`` event"]
    #[inline(always)]
    pub fn nc_b2s14(&self) -> NcB2s14R {
        NcB2s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level of the ``aowkupint`` event"]
    #[inline(always)]
    pub fn aowkupint(&self) -> AowkupintR {
        AowkupintR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level of the ``qfcirq`` event"]
    #[inline(always)]
    pub fn qfcirq(&mut self) -> QfcirqW<'_, EvStatusSpec> {
        QfcirqW::new(self, 0)
    }
    #[doc = "Bit 1 - Level of the ``mdmairq`` event"]
    #[inline(always)]
    pub fn mdmairq(&mut self) -> MdmairqW<'_, EvStatusSpec> {
        MdmairqW::new(self, 1)
    }
    #[doc = "Bit 2 - Level of the ``mbox_irq_available`` event"]
    #[inline(always)]
    pub fn mbox_irq_available(&mut self) -> MboxIrqAvailableW<'_, EvStatusSpec> {
        MboxIrqAvailableW::new(self, 2)
    }
    #[doc = "Bit 3 - Level of the ``mbox_irq_abort_init`` event"]
    #[inline(always)]
    pub fn mbox_irq_abort_init(&mut self) -> MboxIrqAbortInitW<'_, EvStatusSpec> {
        MboxIrqAbortInitW::new(self, 3)
    }
    #[doc = "Bit 4 - Level of the ``mbox_irq_done`` event"]
    #[inline(always)]
    pub fn mbox_irq_done(&mut self) -> MboxIrqDoneW<'_, EvStatusSpec> {
        MboxIrqDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - Level of the ``mbox_irq_error`` event"]
    #[inline(always)]
    pub fn mbox_irq_error(&mut self) -> MboxIrqErrorW<'_, EvStatusSpec> {
        MboxIrqErrorW::new(self, 5)
    }
    #[doc = "Bit 6 - Level of the ``nc_b2s6`` event"]
    #[inline(always)]
    pub fn nc_b2s6(&mut self) -> NcB2s6W<'_, EvStatusSpec> {
        NcB2s6W::new(self, 6)
    }
    #[doc = "Bit 7 - Level of the ``nc_b2s7`` event"]
    #[inline(always)]
    pub fn nc_b2s7(&mut self) -> NcB2s7W<'_, EvStatusSpec> {
        NcB2s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Level of the ``nc_b2s8`` event"]
    #[inline(always)]
    pub fn nc_b2s8(&mut self) -> NcB2s8W<'_, EvStatusSpec> {
        NcB2s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Level of the ``nc_b2s9`` event"]
    #[inline(always)]
    pub fn nc_b2s9(&mut self) -> NcB2s9W<'_, EvStatusSpec> {
        NcB2s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Level of the ``nc_b2s10`` event"]
    #[inline(always)]
    pub fn nc_b2s10(&mut self) -> NcB2s10W<'_, EvStatusSpec> {
        NcB2s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Level of the ``nc_b2s11`` event"]
    #[inline(always)]
    pub fn nc_b2s11(&mut self) -> NcB2s11W<'_, EvStatusSpec> {
        NcB2s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Level of the ``nc_b2s12`` event"]
    #[inline(always)]
    pub fn nc_b2s12(&mut self) -> NcB2s12W<'_, EvStatusSpec> {
        NcB2s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Level of the ``nc_b2s13`` event"]
    #[inline(always)]
    pub fn nc_b2s13(&mut self) -> NcB2s13W<'_, EvStatusSpec> {
        NcB2s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Level of the ``nc_b2s14`` event"]
    #[inline(always)]
    pub fn nc_b2s14(&mut self) -> NcB2s14W<'_, EvStatusSpec> {
        NcB2s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Level of the ``aowkupint`` event"]
    #[inline(always)]
    pub fn aowkupint(&mut self) -> AowkupintW<'_, EvStatusSpec> {
        AowkupintW::new(self, 15)
    }
}
#[doc = "`1` when a \"aowkupint\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
