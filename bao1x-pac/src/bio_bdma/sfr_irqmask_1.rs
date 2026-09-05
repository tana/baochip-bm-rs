#[doc = "Register `SFR_IRQMASK_1` reader"]
pub type R = crate::R<SfrIrqmask1Spec>;
#[doc = "Register `SFR_IRQMASK_1` writer"]
pub type W = crate::W<SfrIrqmask1Spec>;
#[doc = "Field `sfr_irqmask_1` reader - sfr_irqmask_1 read/write control register"]
pub type SfrIrqmask1R = crate::FieldReader<u32>;
#[doc = "Field `sfr_irqmask_1` writer - sfr_irqmask_1 read/write control register"]
pub type SfrIrqmask1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_irqmask_1 read/write control register"]
    #[inline(always)]
    pub fn sfr_irqmask_1(&self) -> SfrIrqmask1R {
        SfrIrqmask1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_irqmask_1 read/write control register"]
    #[inline(always)]
    pub fn sfr_irqmask_1(&mut self) -> SfrIrqmask1W<'_, SfrIrqmask1Spec> {
        SfrIrqmask1W::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L522 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L522>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_irqmask_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_irqmask_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIrqmask1Spec;
impl crate::RegisterSpec for SfrIrqmask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_irqmask_1::R`](R) reader structure"]
impl crate::Readable for SfrIrqmask1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_irqmask_1::W`](W) writer structure"]
impl crate::Writable for SfrIrqmask1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IRQMASK_1 to value 0"]
impl crate::Resettable for SfrIrqmask1Spec {}
