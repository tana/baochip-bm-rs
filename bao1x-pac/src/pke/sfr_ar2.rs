#[doc = "Register `SFR_AR2` reader"]
pub type R = crate::R<SfrAr2Spec>;
#[doc = "Register `SFR_AR2` writer"]
pub type W = crate::W<SfrAr2Spec>;
#[doc = "Field `sfr_ar2` reader - sfr_ar2 performs action on write of value: 0xff"]
pub type SfrAr2R = crate::FieldReader<u32>;
#[doc = "Field `sfr_ar2` writer - sfr_ar2 performs action on write of value: 0xff"]
pub type SfrAr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_ar2 performs action on write of value: 0xff"]
    #[inline(always)]
    pub fn sfr_ar2(&self) -> SfrAr2R {
        SfrAr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_ar2 performs action on write of value: 0xff"]
    #[inline(always)]
    pub fn sfr_ar2(&mut self) -> SfrAr2W<'_, SfrAr2Spec> {
        SfrAr2W::new(self, 0)
    }
}
#[doc = "See `pke.sv#L296 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L296>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrAr2Spec;
impl crate::RegisterSpec for SfrAr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ar2::R`](R) reader structure"]
impl crate::Readable for SfrAr2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ar2::W`](W) writer structure"]
impl crate::Writable for SfrAr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_AR2 to value 0"]
impl crate::Resettable for SfrAr2Spec {}
