#[doc = "Register `SFR_SEGPTR_CR_SEGCFG2` reader"]
pub type R = crate::R<SfrSegptrCrSegcfg2Spec>;
#[doc = "Register `SFR_SEGPTR_CR_SEGCFG2` writer"]
pub type W = crate::W<SfrSegptrCrSegcfg2Spec>;
#[doc = "Field `cr_segcfg2` reader - cr_segcfg read/write control register"]
pub type CrSegcfg2R = crate::FieldReader<u32>;
#[doc = "Field `cr_segcfg2` writer - cr_segcfg read/write control register"]
pub type CrSegcfg2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - cr_segcfg read/write control register"]
    #[inline(always)]
    pub fn cr_segcfg2(&self) -> CrSegcfg2R {
        CrSegcfg2R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - cr_segcfg read/write control register"]
    #[inline(always)]
    pub fn cr_segcfg2(&mut self) -> CrSegcfg2W<'_, SfrSegptrCrSegcfg2Spec> {
        CrSegcfg2W::new(self, 0)
    }
}
#[doc = "See `alu.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_segptr_cr_segcfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_segptr_cr_segcfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSegptrCrSegcfg2Spec;
impl crate::RegisterSpec for SfrSegptrCrSegcfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_segptr_cr_segcfg2::R`](R) reader structure"]
impl crate::Readable for SfrSegptrCrSegcfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_segptr_cr_segcfg2::W`](W) writer structure"]
impl crate::Writable for SfrSegptrCrSegcfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SEGPTR_CR_SEGCFG2 to value 0"]
impl crate::Resettable for SfrSegptrCrSegcfg2Spec {}
