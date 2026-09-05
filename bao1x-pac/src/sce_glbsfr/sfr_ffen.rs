#[doc = "Register `SFR_FFEN` reader"]
pub type R = crate::R<SfrFfenSpec>;
#[doc = "Register `SFR_FFEN` writer"]
pub type W = crate::W<SfrFfenSpec>;
#[doc = "Field `cr_ffen` reader - cr_ffen read/write control register"]
pub type CrFfenR = crate::FieldReader;
#[doc = "Field `cr_ffen` writer - cr_ffen read/write control register"]
pub type CrFfenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - cr_ffen read/write control register"]
    #[inline(always)]
    pub fn cr_ffen(&self) -> CrFfenR {
        CrFfenR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - cr_ffen read/write control register"]
    #[inline(always)]
    pub fn cr_ffen(&mut self) -> CrFfenW<'_, SfrFfenSpec> {
        CrFfenW::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L90 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L90>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFfenSpec;
impl crate::RegisterSpec for SfrFfenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ffen::R`](R) reader structure"]
impl crate::Readable for SfrFfenSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ffen::W`](W) writer structure"]
impl crate::Writable for SfrFfenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FFEN to value 0"]
impl crate::Resettable for SfrFfenSpec {}
