#[doc = "Register `SFR_IRQMASK_2` reader"]
pub type R = crate::R<SfrIrqmask2Spec>;
#[doc = "Register `SFR_IRQMASK_2` writer"]
pub type W = crate::W<SfrIrqmask2Spec>;
#[doc = "Field `sfr_irqmask_2` reader - sfr_irqmask_2 read/write control register"]
pub type SfrIrqmask2R = crate::FieldReader<u32>;
#[doc = "Field `sfr_irqmask_2` writer - sfr_irqmask_2 read/write control register"]
pub type SfrIrqmask2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_irqmask_2 read/write control register"]
    #[inline(always)]
    pub fn sfr_irqmask_2(&self) -> SfrIrqmask2R {
        SfrIrqmask2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_irqmask_2 read/write control register"]
    #[inline(always)]
    pub fn sfr_irqmask_2(&mut self) -> SfrIrqmask2W<'_, SfrIrqmask2Spec> {
        SfrIrqmask2W::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L523 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L523>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_irqmask_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_irqmask_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIrqmask2Spec;
impl crate::RegisterSpec for SfrIrqmask2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_irqmask_2::R`](R) reader structure"]
impl crate::Readable for SfrIrqmask2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_irqmask_2::W`](W) writer structure"]
impl crate::Writable for SfrIrqmask2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IRQMASK_2 to value 0"]
impl crate::Resettable for SfrIrqmask2Spec {}
