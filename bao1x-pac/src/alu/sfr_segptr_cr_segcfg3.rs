#[doc = "Register `SFR_SEGPTR_CR_SEGCFG3` reader"]
pub type R = crate::R<SfrSegptrCrSegcfg3Spec>;
#[doc = "Register `SFR_SEGPTR_CR_SEGCFG3` writer"]
pub type W = crate::W<SfrSegptrCrSegcfg3Spec>;
#[doc = "Field `cr_segcfg3` reader - cr_segcfg read/write control register"]
pub type CrSegcfg3R = crate::FieldReader<u32>;
#[doc = "Field `cr_segcfg3` writer - cr_segcfg read/write control register"]
pub type CrSegcfg3W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - cr_segcfg read/write control register"]
    #[inline(always)]
    pub fn cr_segcfg3(&self) -> CrSegcfg3R {
        CrSegcfg3R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - cr_segcfg read/write control register"]
    #[inline(always)]
    pub fn cr_segcfg3(&mut self) -> CrSegcfg3W<'_, SfrSegptrCrSegcfg3Spec> {
        CrSegcfg3W::new(self, 0)
    }
}
#[doc = "See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_cr_segcfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_cr_segcfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSegptrCrSegcfg3Spec;
impl crate::RegisterSpec for SfrSegptrCrSegcfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_segptr_cr_segcfg3::R`](R) reader structure"]
impl crate::Readable for SfrSegptrCrSegcfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_segptr_cr_segcfg3::W`](W) writer structure"]
impl crate::Writable for SfrSegptrCrSegcfg3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEGPTR_CR_SEGCFG3 to value 0"]
impl crate::Resettable for SfrSegptrCrSegcfg3Spec {}
