#[doc = "Register `EV_ENABLE` reader"]
pub type R = crate::R<EvEnableSpec>;
#[doc = "Register `EV_ENABLE` writer"]
pub type W = crate::W<EvEnableSpec>;
#[doc = "Field `qfcirq` reader - Write a ``1`` to enable the ``qfcirq`` Event"]
pub type QfcirqR = crate::BitReader;
#[doc = "Field `qfcirq` writer - Write a ``1`` to enable the ``qfcirq`` Event"]
pub type QfcirqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mdmairq` reader - Write a ``1`` to enable the ``mdmairq`` Event"]
pub type MdmairqR = crate::BitReader;
#[doc = "Field `mdmairq` writer - Write a ``1`` to enable the ``mdmairq`` Event"]
pub type MdmairqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_available` reader - Write a ``1`` to enable the ``mbox_irq_available`` Event"]
pub type MboxIrqAvailableR = crate::BitReader;
#[doc = "Field `mbox_irq_available` writer - Write a ``1`` to enable the ``mbox_irq_available`` Event"]
pub type MboxIrqAvailableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_abort_init` reader - Write a ``1`` to enable the ``mbox_irq_abort_init`` Event"]
pub type MboxIrqAbortInitR = crate::BitReader;
#[doc = "Field `mbox_irq_abort_init` writer - Write a ``1`` to enable the ``mbox_irq_abort_init`` Event"]
pub type MboxIrqAbortInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_done` reader - Write a ``1`` to enable the ``mbox_irq_done`` Event"]
pub type MboxIrqDoneR = crate::BitReader;
#[doc = "Field `mbox_irq_done` writer - Write a ``1`` to enable the ``mbox_irq_done`` Event"]
pub type MboxIrqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mbox_irq_error` reader - Write a ``1`` to enable the ``mbox_irq_error`` Event"]
pub type MboxIrqErrorR = crate::BitReader;
#[doc = "Field `mbox_irq_error` writer - Write a ``1`` to enable the ``mbox_irq_error`` Event"]
pub type MboxIrqErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s6` reader - Write a ``1`` to enable the ``nc_b2s6`` Event"]
pub type NcB2s6R = crate::BitReader;
#[doc = "Field `nc_b2s6` writer - Write a ``1`` to enable the ``nc_b2s6`` Event"]
pub type NcB2s6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s7` reader - Write a ``1`` to enable the ``nc_b2s7`` Event"]
pub type NcB2s7R = crate::BitReader;
#[doc = "Field `nc_b2s7` writer - Write a ``1`` to enable the ``nc_b2s7`` Event"]
pub type NcB2s7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s8` reader - Write a ``1`` to enable the ``nc_b2s8`` Event"]
pub type NcB2s8R = crate::BitReader;
#[doc = "Field `nc_b2s8` writer - Write a ``1`` to enable the ``nc_b2s8`` Event"]
pub type NcB2s8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s9` reader - Write a ``1`` to enable the ``nc_b2s9`` Event"]
pub type NcB2s9R = crate::BitReader;
#[doc = "Field `nc_b2s9` writer - Write a ``1`` to enable the ``nc_b2s9`` Event"]
pub type NcB2s9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s10` reader - Write a ``1`` to enable the ``nc_b2s10`` Event"]
pub type NcB2s10R = crate::BitReader;
#[doc = "Field `nc_b2s10` writer - Write a ``1`` to enable the ``nc_b2s10`` Event"]
pub type NcB2s10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s11` reader - Write a ``1`` to enable the ``nc_b2s11`` Event"]
pub type NcB2s11R = crate::BitReader;
#[doc = "Field `nc_b2s11` writer - Write a ``1`` to enable the ``nc_b2s11`` Event"]
pub type NcB2s11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s12` reader - Write a ``1`` to enable the ``nc_b2s12`` Event"]
pub type NcB2s12R = crate::BitReader;
#[doc = "Field `nc_b2s12` writer - Write a ``1`` to enable the ``nc_b2s12`` Event"]
pub type NcB2s12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s13` reader - Write a ``1`` to enable the ``nc_b2s13`` Event"]
pub type NcB2s13R = crate::BitReader;
#[doc = "Field `nc_b2s13` writer - Write a ``1`` to enable the ``nc_b2s13`` Event"]
pub type NcB2s13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nc_b2s14` reader - Write a ``1`` to enable the ``nc_b2s14`` Event"]
pub type NcB2s14R = crate::BitReader;
#[doc = "Field `nc_b2s14` writer - Write a ``1`` to enable the ``nc_b2s14`` Event"]
pub type NcB2s14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aowkupint` reader - Write a ``1`` to enable the ``aowkupint`` Event"]
pub type AowkupintR = crate::BitReader;
#[doc = "Field `aowkupint` writer - Write a ``1`` to enable the ``aowkupint`` Event"]
pub type AowkupintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``qfcirq`` Event"]
    #[inline(always)]
    pub fn qfcirq(&self) -> QfcirqR {
        QfcirqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``mdmairq`` Event"]
    #[inline(always)]
    pub fn mdmairq(&self) -> MdmairqR {
        MdmairqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``mbox_irq_available`` Event"]
    #[inline(always)]
    pub fn mbox_irq_available(&self) -> MboxIrqAvailableR {
        MboxIrqAvailableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``mbox_irq_abort_init`` Event"]
    #[inline(always)]
    pub fn mbox_irq_abort_init(&self) -> MboxIrqAbortInitR {
        MboxIrqAbortInitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``mbox_irq_done`` Event"]
    #[inline(always)]
    pub fn mbox_irq_done(&self) -> MboxIrqDoneR {
        MboxIrqDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``mbox_irq_error`` Event"]
    #[inline(always)]
    pub fn mbox_irq_error(&self) -> MboxIrqErrorR {
        MboxIrqErrorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``nc_b2s6`` Event"]
    #[inline(always)]
    pub fn nc_b2s6(&self) -> NcB2s6R {
        NcB2s6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b2s7`` Event"]
    #[inline(always)]
    pub fn nc_b2s7(&self) -> NcB2s7R {
        NcB2s7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b2s8`` Event"]
    #[inline(always)]
    pub fn nc_b2s8(&self) -> NcB2s8R {
        NcB2s8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b2s9`` Event"]
    #[inline(always)]
    pub fn nc_b2s9(&self) -> NcB2s9R {
        NcB2s9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b2s10`` Event"]
    #[inline(always)]
    pub fn nc_b2s10(&self) -> NcB2s10R {
        NcB2s10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b2s11`` Event"]
    #[inline(always)]
    pub fn nc_b2s11(&self) -> NcB2s11R {
        NcB2s11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b2s12`` Event"]
    #[inline(always)]
    pub fn nc_b2s12(&self) -> NcB2s12R {
        NcB2s12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b2s13`` Event"]
    #[inline(always)]
    pub fn nc_b2s13(&self) -> NcB2s13R {
        NcB2s13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b2s14`` Event"]
    #[inline(always)]
    pub fn nc_b2s14(&self) -> NcB2s14R {
        NcB2s14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``aowkupint`` Event"]
    #[inline(always)]
    pub fn aowkupint(&self) -> AowkupintR {
        AowkupintR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write a ``1`` to enable the ``qfcirq`` Event"]
    #[inline(always)]
    pub fn qfcirq(&mut self) -> QfcirqW<'_, EvEnableSpec> {
        QfcirqW::new(self, 0)
    }
    #[doc = "Bit 1 - Write a ``1`` to enable the ``mdmairq`` Event"]
    #[inline(always)]
    pub fn mdmairq(&mut self) -> MdmairqW<'_, EvEnableSpec> {
        MdmairqW::new(self, 1)
    }
    #[doc = "Bit 2 - Write a ``1`` to enable the ``mbox_irq_available`` Event"]
    #[inline(always)]
    pub fn mbox_irq_available(&mut self) -> MboxIrqAvailableW<'_, EvEnableSpec> {
        MboxIrqAvailableW::new(self, 2)
    }
    #[doc = "Bit 3 - Write a ``1`` to enable the ``mbox_irq_abort_init`` Event"]
    #[inline(always)]
    pub fn mbox_irq_abort_init(&mut self) -> MboxIrqAbortInitW<'_, EvEnableSpec> {
        MboxIrqAbortInitW::new(self, 3)
    }
    #[doc = "Bit 4 - Write a ``1`` to enable the ``mbox_irq_done`` Event"]
    #[inline(always)]
    pub fn mbox_irq_done(&mut self) -> MboxIrqDoneW<'_, EvEnableSpec> {
        MboxIrqDoneW::new(self, 4)
    }
    #[doc = "Bit 5 - Write a ``1`` to enable the ``mbox_irq_error`` Event"]
    #[inline(always)]
    pub fn mbox_irq_error(&mut self) -> MboxIrqErrorW<'_, EvEnableSpec> {
        MboxIrqErrorW::new(self, 5)
    }
    #[doc = "Bit 6 - Write a ``1`` to enable the ``nc_b2s6`` Event"]
    #[inline(always)]
    pub fn nc_b2s6(&mut self) -> NcB2s6W<'_, EvEnableSpec> {
        NcB2s6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write a ``1`` to enable the ``nc_b2s7`` Event"]
    #[inline(always)]
    pub fn nc_b2s7(&mut self) -> NcB2s7W<'_, EvEnableSpec> {
        NcB2s7W::new(self, 7)
    }
    #[doc = "Bit 8 - Write a ``1`` to enable the ``nc_b2s8`` Event"]
    #[inline(always)]
    pub fn nc_b2s8(&mut self) -> NcB2s8W<'_, EvEnableSpec> {
        NcB2s8W::new(self, 8)
    }
    #[doc = "Bit 9 - Write a ``1`` to enable the ``nc_b2s9`` Event"]
    #[inline(always)]
    pub fn nc_b2s9(&mut self) -> NcB2s9W<'_, EvEnableSpec> {
        NcB2s9W::new(self, 9)
    }
    #[doc = "Bit 10 - Write a ``1`` to enable the ``nc_b2s10`` Event"]
    #[inline(always)]
    pub fn nc_b2s10(&mut self) -> NcB2s10W<'_, EvEnableSpec> {
        NcB2s10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write a ``1`` to enable the ``nc_b2s11`` Event"]
    #[inline(always)]
    pub fn nc_b2s11(&mut self) -> NcB2s11W<'_, EvEnableSpec> {
        NcB2s11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write a ``1`` to enable the ``nc_b2s12`` Event"]
    #[inline(always)]
    pub fn nc_b2s12(&mut self) -> NcB2s12W<'_, EvEnableSpec> {
        NcB2s12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write a ``1`` to enable the ``nc_b2s13`` Event"]
    #[inline(always)]
    pub fn nc_b2s13(&mut self) -> NcB2s13W<'_, EvEnableSpec> {
        NcB2s13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write a ``1`` to enable the ``nc_b2s14`` Event"]
    #[inline(always)]
    pub fn nc_b2s14(&mut self) -> NcB2s14W<'_, EvEnableSpec> {
        NcB2s14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write a ``1`` to enable the ``aowkupint`` Event"]
    #[inline(always)]
    pub fn aowkupint(&mut self) -> AowkupintW<'_, EvEnableSpec> {
        AowkupintW::new(self, 15)
    }
}
#[doc = "`1` when a \"aowkupint\" event occurs. This event uses an `EventSourceFlex` form of triggering\n\nYou can [`read`](crate::Reg::read) this register and get [`ev_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ev_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvEnableSpec;
impl crate::RegisterSpec for EvEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ev_enable::R`](R) reader structure"]
impl crate::Readable for EvEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`ev_enable::W`](W) writer structure"]
impl crate::Writable for EvEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EV_ENABLE to value 0"]
impl crate::Resettable for EvEnableSpec {}
