#[doc = "Register `SFR_MLDRV_CR_MLDRV1` reader"]
pub type R = crate::R<SfrMldrvCrMldrv1Spec>;
#[doc = "Register `SFR_MLDRV_CR_MLDRV1` writer"]
pub type W = crate::W<SfrMldrvCrMldrv1Spec>;
#[doc = "Field `cr_mldrv1` reader - cr_mldrv read/write control register"]
pub type CrMldrv1R = crate::FieldReader<u32>;
#[doc = "Field `cr_mldrv1` writer - cr_mldrv read/write control register"]
pub type CrMldrv1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_mldrv read/write control register"]
    #[inline(always)]
    pub fn cr_mldrv1(&self) -> CrMldrv1R {
        CrMldrv1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_mldrv read/write control register"]
    #[inline(always)]
    pub fn cr_mldrv1(&mut self) -> CrMldrv1W<'_, SfrMldrvCrMldrv1Spec> {
        CrMldrv1W::new(self, 0)
    }
}
#[doc = "See `mesh.sv#L49 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L49>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mldrv_cr_mldrv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mldrv_cr_mldrv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrMldrvCrMldrv1Spec;
impl crate::RegisterSpec for SfrMldrvCrMldrv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_mldrv_cr_mldrv1::R`](R) reader structure"]
impl crate::Readable for SfrMldrvCrMldrv1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_mldrv_cr_mldrv1::W`](W) writer structure"]
impl crate::Writable for SfrMldrvCrMldrv1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_MLDRV_CR_MLDRV1 to value 0"]
impl crate::Resettable for SfrMldrvCrMldrv1Spec {}
