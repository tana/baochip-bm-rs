#[doc = "Register `SFR_IRQMASK_3` reader"]
pub type R = crate::R<SfrIrqmask3Spec>;
#[doc = "Register `SFR_IRQMASK_3` writer"]
pub type W = crate::W<SfrIrqmask3Spec>;
#[doc = "Field `sfr_irqmask_3` reader - sfr_irqmask_3 read/write control register"]
pub type SfrIrqmask3R = crate::FieldReader<u32>;
#[doc = "Field `sfr_irqmask_3` writer - sfr_irqmask_3 read/write control register"]
pub type SfrIrqmask3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_irqmask_3 read/write control register"]
    #[inline(always)]
    pub fn sfr_irqmask_3(&self) -> SfrIrqmask3R {
        SfrIrqmask3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_irqmask_3 read/write control register"]
    #[inline(always)]
    pub fn sfr_irqmask_3(&mut self) -> SfrIrqmask3W<'_, SfrIrqmask3Spec> {
        SfrIrqmask3W::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L524 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L524>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_irqmask_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_irqmask_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIrqmask3Spec;
impl crate::RegisterSpec for SfrIrqmask3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_irqmask_3::R`](R) reader structure"]
impl crate::Readable for SfrIrqmask3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_irqmask_3::W`](W) writer structure"]
impl crate::Writable for SfrIrqmask3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IRQMASK_3 to value 0"]
impl crate::Resettable for SfrIrqmask3Spec {}
