#[doc = "Register `SFR_GCRST_GLUERST0` reader"]
pub type R = crate::R<SfrGcrstGluerst0Spec>;
#[doc = "Register `SFR_GCRST_GLUERST0` writer"]
pub type W = crate::W<SfrGcrstGluerst0Spec>;
#[doc = "Field `gluerst0` reader - gluerst read/write control register"]
pub type Gluerst0R = crate::FieldReader<u32>;
#[doc = "Field `gluerst0` writer - gluerst read/write control register"]
pub type Gluerst0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - gluerst read/write control register"]
    #[inline(always)]
    pub fn gluerst0(&self) -> Gluerst0R {
        Gluerst0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - gluerst read/write control register"]
    #[inline(always)]
    pub fn gluerst0(&mut self) -> Gluerst0W<'_, SfrGcrstGluerst0Spec> {
        Gluerst0W::new(self, 0)
    }
}
#[doc = "See `gluechain.sv#L46 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/sec/rtl/gluechain.sv#L46>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_gcrst_gluerst0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_gcrst_gluerst0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrGcrstGluerst0Spec;
impl crate::RegisterSpec for SfrGcrstGluerst0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_gcrst_gluerst0::R`](R) reader structure"]
impl crate::Readable for SfrGcrstGluerst0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_gcrst_gluerst0::W`](W) writer structure"]
impl crate::Writable for SfrGcrstGluerst0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_GCRST_GLUERST0 to value 0"]
impl crate::Resettable for SfrGcrstGluerst0Spec {}
