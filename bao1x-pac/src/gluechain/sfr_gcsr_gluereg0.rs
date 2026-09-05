#[doc = "Register `SFR_GCSR_GLUEREG0` reader"]
pub type R = crate::R<SfrGcsrGluereg0Spec>;
#[doc = "Register `SFR_GCSR_GLUEREG0` writer"]
pub type W = crate::W<SfrGcsrGluereg0Spec>;
#[doc = "Field `gluereg0` reader - gluereg read only status register"]
pub type Gluereg0R = crate::FieldReader<u32>;
#[doc = "Field `gluereg0` writer - gluereg read only status register"]
pub type Gluereg0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - gluereg read only status register"]
    #[inline(always)]
    pub fn gluereg0(&self) -> Gluereg0R {
        Gluereg0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - gluereg read only status register"]
    #[inline(always)]
    pub fn gluereg0(&mut self) -> Gluereg0W<'_, SfrGcsrGluereg0Spec> {
        Gluereg0W::new(self, 0)
    }
}
#[doc = "See `gluechain.sv#L45 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L45>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gcsr_gluereg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gcsr_gluereg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGcsrGluereg0Spec;
impl crate::RegisterSpec for SfrGcsrGluereg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gcsr_gluereg0::R`](R) reader structure"]
impl crate::Readable for SfrGcsrGluereg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gcsr_gluereg0::W`](W) writer structure"]
impl crate::Writable for SfrGcsrGluereg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GCSR_GLUEREG0 to value 0"]
impl crate::Resettable for SfrGcsrGluereg0Spec {}
