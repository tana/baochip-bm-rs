#[doc = "Register `SFR_FILTER_BOUNDS_0` reader"]
pub type R = crate::R<SfrFilterBounds0Spec>;
#[doc = "Register `SFR_FILTER_BOUNDS_0` writer"]
pub type W = crate::W<SfrFilterBounds0Spec>;
#[doc = "Field `filter_bounds` reader - filter_bounds read/write control register"]
pub type FilterBoundsR = crate::FieldReader<u32>;
#[doc = "Field `filter_bounds` writer - filter_bounds read/write control register"]
pub type FilterBoundsW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - filter_bounds read/write control register"]
    #[inline(always)]
    pub fn filter_bounds(&self) -> FilterBoundsR {
        FilterBoundsR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - filter_bounds read/write control register"]
    #[inline(always)]
    pub fn filter_bounds(&mut self) -> FilterBoundsW<'_, SfrFilterBounds0Spec> {
        FilterBoundsW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L594 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L594>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_filter_bounds_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_filter_bounds_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFilterBounds0Spec;
impl crate::RegisterSpec for SfrFilterBounds0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_filter_bounds_0::R`](R) reader structure"]
impl crate::Readable for SfrFilterBounds0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_filter_bounds_0::W`](W) writer structure"]
impl crate::Writable for SfrFilterBounds0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FILTER_BOUNDS_0 to value 0"]
impl crate::Resettable for SfrFilterBounds0Spec {}
