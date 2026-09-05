#[doc = "Register `SFR_MLDRV_CR_MLDRV0` reader"]
pub type R = crate::R<SfrMldrvCrMldrv0Spec>;
#[doc = "Register `SFR_MLDRV_CR_MLDRV0` writer"]
pub type W = crate::W<SfrMldrvCrMldrv0Spec>;
#[doc = "Field `cr_mldrv0` reader - cr_mldrv read/write control register"]
pub type CrMldrv0R = crate::FieldReader<u32>;
#[doc = "Field `cr_mldrv0` writer - cr_mldrv read/write control register"]
pub type CrMldrv0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_mldrv read/write control register"]
    #[inline(always)]
    pub fn cr_mldrv0(&self) -> CrMldrv0R {
        CrMldrv0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_mldrv read/write control register"]
    #[inline(always)]
    pub fn cr_mldrv0(&mut self) -> CrMldrv0W<'_, SfrMldrvCrMldrv0Spec> {
        CrMldrv0W::new(self, 0)
    }
}
#[doc = "See `mesh.sv#L49 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/se c/rtl/mesh.sv#L49>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mldrv_cr_mldrv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mldrv_cr_mldrv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrMldrvCrMldrv0Spec;
impl crate::RegisterSpec for SfrMldrvCrMldrv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_mldrv_cr_mldrv0::R`](R) reader structure"]
impl crate::Readable for SfrMldrvCrMldrv0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_mldrv_cr_mldrv0::W`](W) writer structure"]
impl crate::Writable for SfrMldrvCrMldrv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_MLDRV_CR_MLDRV0 to value 0"]
impl crate::Resettable for SfrMldrvCrMldrv0Spec {}
