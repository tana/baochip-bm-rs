#[doc = "Register `SFR_RRCSR_SET1` reader"]
pub type R = crate::R<SfrRrcsrSet1Spec>;
#[doc = "Register `SFR_RRCSR_SET1` writer"]
pub type W = crate::W<SfrRrcsrSet1Spec>;
#[doc = "Field `trc_set_failure` reader - trc_set_failure read only status register"]
pub type TrcSetFailureR = crate::FieldReader<u32>;
#[doc = "Field `trc_set_failure` writer - trc_set_failure read only status register"]
pub type TrcSetFailureW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - trc_set_failure read only status register"]
    #[inline(always)]
    pub fn trc_set_failure(&self) -> TrcSetFailureR {
        TrcSetFailureR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - trc_set_failure read only status register"]
    #[inline(always)]
    pub fn trc_set_failure(&mut self) -> TrcSetFailureW<'_, SfrRrcsrSet1Spec> {
        TrcSetFailureW::new(self, 0)
    }
}
#[doc = "See `rrc.sv#L267 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L267>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr_set1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr_set1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcsrSet1Spec;
impl crate::RegisterSpec for SfrRrcsrSet1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrcsr_set1::R`](R) reader structure"]
impl crate::Readable for SfrRrcsrSet1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrcsr_set1::W`](W) writer structure"]
impl crate::Writable for SfrRrcsrSet1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCSR_SET1 to value 0"]
impl crate::Resettable for SfrRrcsrSet1Spec {}
