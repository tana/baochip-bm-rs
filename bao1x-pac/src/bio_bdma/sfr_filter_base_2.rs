#[doc = "Register `SFR_FILTER_BASE_2` reader"]
pub type R = crate::R<SfrFilterBase2Spec>;
#[doc = "Register `SFR_FILTER_BASE_2` writer"]
pub type W = crate::W<SfrFilterBase2Spec>;
#[doc = "Field `filter_base` reader - filter_base read/write control register"]
pub type FilterBaseR = crate::FieldReader<u32>;
#[doc = "Field `filter_base` writer - filter_base read/write control register"]
pub type FilterBaseW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - filter_base read/write control register"]
    #[inline(always)]
    pub fn filter_base(&self) -> FilterBaseR {
        FilterBaseR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - filter_base read/write control register"]
    #[inline(always)]
    pub fn filter_base(&mut self) -> FilterBaseW<'_, SfrFilterBase2Spec> {
        FilterBaseW::new(self, 0)
    }
}
#[doc = "See `bio_bdma.sv#L597 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L597>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_filter_base_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_filter_base_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFilterBase2Spec;
impl crate::RegisterSpec for SfrFilterBase2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_filter_base_2::R`](R) reader structure"]
impl crate::Readable for SfrFilterBase2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_filter_base_2::W`](W) writer structure"]
impl crate::Writable for SfrFilterBase2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FILTER_BASE_2 to value 0"]
impl crate::Resettable for SfrFilterBase2Spec {}
