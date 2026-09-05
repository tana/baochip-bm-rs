#[doc = "Register `SFR_VDMASK1` reader"]
pub type R = crate::R<SfrVdmask1Spec>;
#[doc = "Register `SFR_VDMASK1` writer"]
pub type W = crate::W<SfrVdmask1Spec>;
#[doc = "Field `cr_vdmask1` reader - cr_vdmask1 read/write control register"]
pub type CrVdmask1R = crate::FieldReader;
#[doc = "Field `cr_vdmask1` writer - cr_vdmask1 read/write control register"]
pub type CrVdmask1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cr_vdmask1 read/write control register"]
    #[inline(always)]
    pub fn cr_vdmask1(&self) -> CrVdmask1R {
        CrVdmask1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cr_vdmask1 read/write control register"]
    #[inline(always)]
    pub fn cr_vdmask1(&mut self) -> CrVdmask1W<'_, SfrVdmask1Spec> {
        CrVdmask1W::new(self, 0)
    }
}
#[doc = "See `sensorc.sv#L64 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /sec/rtl/sensorc.sv#L64>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vdmask1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vdmask1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrVdmask1Spec;
impl crate::RegisterSpec for SfrVdmask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_vdmask1::R`](R) reader structure"]
impl crate::Readable for SfrVdmask1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_vdmask1::W`](W) writer structure"]
impl crate::Writable for SfrVdmask1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_VDMASK1 to value 0"]
impl crate::Resettable for SfrVdmask1Spec {}
