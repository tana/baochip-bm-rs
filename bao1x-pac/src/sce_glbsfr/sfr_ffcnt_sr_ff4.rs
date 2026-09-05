#[doc = "Register `SFR_FFCNT_SR_FF4` reader"]
pub type R = crate::R<SfrFfcntSrFf4Spec>;
#[doc = "Register `SFR_FFCNT_SR_FF4` writer"]
pub type W = crate::W<SfrFfcntSrFf4Spec>;
#[doc = "Field `sr_ff4` reader - sr_ff read only status register"]
pub type SrFf4R = crate::FieldReader<u16>;
#[doc = "Field `sr_ff4` writer - sr_ff read only status register"]
pub type SrFf4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sr_ff read only status register"]
    #[inline(always)]
    pub fn sr_ff4(&self) -> SrFf4R {
        SrFf4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sr_ff read only status register"]
    #[inline(always)]
    pub fn sr_ff4(&mut self) -> SrFf4W<'_, SfrFfcntSrFf4Spec> {
        SrFf4W::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffcnt_sr_ff4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffcnt_sr_ff4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFfcntSrFf4Spec;
impl crate::RegisterSpec for SfrFfcntSrFf4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ffcnt_sr_ff4::R`](R) reader structure"]
impl crate::Readable for SfrFfcntSrFf4Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ffcnt_sr_ff4::W`](W) writer structure"]
impl crate::Writable for SfrFfcntSrFf4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FFCNT_SR_FF4 to value 0"]
impl crate::Resettable for SfrFfcntSrFf4Spec {}
