#[doc = "Register `SFR_MLSR_SR_MLSR4` reader"]
pub type R = crate::R<SfrMlsrSrMlsr4Spec>;
#[doc = "Register `SFR_MLSR_SR_MLSR4` writer"]
pub type W = crate::W<SfrMlsrSrMlsr4Spec>;
#[doc = "Field `sr_mlsr4` reader - sr_mlsr read only status register"]
pub type SrMlsr4R = crate::FieldReader<u32>;
#[doc = "Field `sr_mlsr4` writer - sr_mlsr read only status register"]
pub type SrMlsr4W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sr_mlsr read only status register"]
    #[inline(always)]
    pub fn sr_mlsr4(&self) -> SrMlsr4R {
        SrMlsr4R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sr_mlsr read only status register"]
    #[inline(always)]
    pub fn sr_mlsr4(&mut self) -> SrMlsr4W<'_, SfrMlsrSrMlsr4Spec> {
        SrMlsr4W::new(self, 0)
    }
}
#[doc = "See `mesh.sv#L51 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L51>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mlsr_sr_mlsr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mlsr_sr_mlsr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrMlsrSrMlsr4Spec;
impl crate::RegisterSpec for SfrMlsrSrMlsr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_mlsr_sr_mlsr4::R`](R) reader structure"]
impl crate::Readable for SfrMlsrSrMlsr4Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_mlsr_sr_mlsr4::W`](W) writer structure"]
impl crate::Writable for SfrMlsrSrMlsr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_MLSR_SR_MLSR4 to value 0"]
impl crate::Resettable for SfrMlsrSrMlsr4Spec {}
