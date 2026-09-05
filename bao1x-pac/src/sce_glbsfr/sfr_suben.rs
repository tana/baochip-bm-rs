#[doc = "Register `SFR_SUBEN` reader"]
pub type R = crate::R<SfrSubenSpec>;
#[doc = "Register `SFR_SUBEN` writer"]
pub type W = crate::W<SfrSubenSpec>;
#[doc = "Field `cr_suben` reader - cr_suben read/write control register"]
pub type CrSubenR = crate::FieldReader<u16>;
#[doc = "Field `cr_suben` writer - cr_suben read/write control register"]
pub type CrSubenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cr_suben read/write control register"]
    #[inline(always)]
    pub fn cr_suben(&self) -> CrSubenR {
        CrSubenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cr_suben read/write control register"]
    #[inline(always)]
    pub fn cr_suben(&mut self) -> CrSubenW<'_, SfrSubenSpec> {
        CrSubenW::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L76 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L76>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_suben::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_suben::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSubenSpec;
impl crate::RegisterSpec for SfrSubenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_suben::R`](R) reader structure"]
impl crate::Readable for SfrSubenSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_suben::W`](W) writer structure"]
impl crate::Writable for SfrSubenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SUBEN to value 0"]
impl crate::Resettable for SfrSubenSpec {}
