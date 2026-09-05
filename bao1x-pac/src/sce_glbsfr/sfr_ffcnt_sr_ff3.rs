#[doc = "Register `SFR_FFCNT_SR_FF3` reader"]
pub type R = crate::R<SfrFfcntSrFf3Spec>;
#[doc = "Register `SFR_FFCNT_SR_FF3` writer"]
pub type W = crate::W<SfrFfcntSrFf3Spec>;
#[doc = "Field `sr_ff3` reader - sr_ff read only status register"]
pub type SrFf3R = crate::FieldReader<u16>;
#[doc = "Field `sr_ff3` writer - sr_ff read only status register"]
pub type SrFf3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sr_ff read only status register"]
    #[inline(always)]
    pub fn sr_ff3(&self) -> SrFf3R {
        SrFf3R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sr_ff read only status register"]
    #[inline(always)]
    pub fn sr_ff3(&mut self) -> SrFf3W<'_, SfrFfcntSrFf3Spec> {
        SrFf3W::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffcnt_sr_ff3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffcnt_sr_ff3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFfcntSrFf3Spec;
impl crate::RegisterSpec for SfrFfcntSrFf3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ffcnt_sr_ff3::R`](R) reader structure"]
impl crate::Readable for SfrFfcntSrFf3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ffcnt_sr_ff3::W`](W) writer structure"]
impl crate::Writable for SfrFfcntSrFf3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FFCNT_SR_FF3 to value 0"]
impl crate::Resettable for SfrFfcntSrFf3Spec {}
