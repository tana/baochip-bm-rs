#[doc = "Register `SFR_MLSR_SR_MLSR0` reader"]
pub type R = crate::R<SfrMlsrSrMlsr0Spec>;
#[doc = "Register `SFR_MLSR_SR_MLSR0` writer"]
pub type W = crate::W<SfrMlsrSrMlsr0Spec>;
#[doc = "Field `sr_mlsr0` reader - sr_mlsr read only status register"]
pub type SrMlsr0R = crate::FieldReader<u32>;
#[doc = "Field `sr_mlsr0` writer - sr_mlsr read only status register"]
pub type SrMlsr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sr_mlsr read only status register"]
    #[inline(always)]
    pub fn sr_mlsr0(&self) -> SrMlsr0R {
        SrMlsr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sr_mlsr read only status register"]
    #[inline(always)]
    pub fn sr_mlsr0(&mut self) -> SrMlsr0W<'_, SfrMlsrSrMlsr0Spec> {
        SrMlsr0W::new(self, 0)
    }
}
#[doc = "See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlsr_sr_mlsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlsr_sr_mlsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrMlsrSrMlsr0Spec;
impl crate::RegisterSpec for SfrMlsrSrMlsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_mlsr_sr_mlsr0::R`](R) reader structure"]
impl crate::Readable for SfrMlsrSrMlsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_mlsr_sr_mlsr0::W`](W) writer structure"]
impl crate::Writable for SfrMlsrSrMlsr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_MLSR_SR_MLSR0 to value 0"]
impl crate::Resettable for SfrMlsrSrMlsr0Spec {}
