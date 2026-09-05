#[doc = "Register `SFR_FFCNT_SR_FF0` reader"]
pub type R = crate::R<SfrFfcntSrFf0Spec>;
#[doc = "Register `SFR_FFCNT_SR_FF0` writer"]
pub type W = crate::W<SfrFfcntSrFf0Spec>;
#[doc = "Field `sr_ff0` reader - sr_ff read only status register"]
pub type SrFf0R = crate::FieldReader<u16>;
#[doc = "Field `sr_ff0` writer - sr_ff read only status register"]
pub type SrFf0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sr_ff read only status register"]
    #[inline(always)]
    pub fn sr_ff0(&self) -> SrFf0R {
        SrFf0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sr_ff read only status register"]
    #[inline(always)]
    pub fn sr_ff0(&mut self) -> SrFf0W<'_, SfrFfcntSrFf0Spec> {
        SrFf0W::new(self, 0)
    }
}
#[doc = "See `sce_glbsfra.sv#L91 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/sce_glbsfra.sv#L91>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ffcnt_sr_ff0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ffcnt_sr_ff0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFfcntSrFf0Spec;
impl crate::RegisterSpec for SfrFfcntSrFf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ffcnt_sr_ff0::R`](R) reader structure"]
impl crate::Readable for SfrFfcntSrFf0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ffcnt_sr_ff0::W`](W) writer structure"]
impl crate::Writable for SfrFfcntSrFf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FFCNT_SR_FF0 to value 0"]
impl crate::Resettable for SfrFfcntSrFf0Spec {}
