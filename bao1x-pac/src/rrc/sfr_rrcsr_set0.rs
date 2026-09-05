#[doc = "Register `SFR_RRCSR_SET0` reader"]
pub type R = crate::R<SfrRrcsrSet0Spec>;
#[doc = "Register `SFR_RRCSR_SET0` writer"]
pub type W = crate::W<SfrRrcsrSet0Spec>;
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
    pub fn trc_set_failure(&mut self) -> TrcSetFailureW<'_, SfrRrcsrSet0Spec> {
        TrcSetFailureW::new(self, 0)
    }
}
#[doc = "See `rrc.sv#L266 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L266>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr_set0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr_set0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcsrSet0Spec;
impl crate::RegisterSpec for SfrRrcsrSet0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrcsr_set0::R`](R) reader structure"]
impl crate::Readable for SfrRrcsrSet0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrcsr_set0::W`](W) writer structure"]
impl crate::Writable for SfrRrcsrSet0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCSR_SET0 to value 0"]
impl crate::Resettable for SfrRrcsrSet0Spec {}
