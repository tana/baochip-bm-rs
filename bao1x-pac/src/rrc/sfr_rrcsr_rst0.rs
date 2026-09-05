#[doc = "Register `SFR_RRCSR_RST0` reader"]
pub type R = crate::R<SfrRrcsrRst0Spec>;
#[doc = "Register `SFR_RRCSR_RST0` writer"]
pub type W = crate::W<SfrRrcsrRst0Spec>;
#[doc = "Field `trc_reset_failure` reader - trc_reset_failure read only status register"]
pub type TrcResetFailureR = crate::FieldReader<u32>;
#[doc = "Field `trc_reset_failure` writer - trc_reset_failure read only status register"]
pub type TrcResetFailureW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - trc_reset_failure read only status register"]
    #[inline(always)]
    pub fn trc_reset_failure(&self) -> TrcResetFailureR {
        TrcResetFailureR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - trc_reset_failure read only status register"]
    #[inline(always)]
    pub fn trc_reset_failure(&mut self) -> TrcResetFailureW<'_, SfrRrcsrRst0Spec> {
        TrcResetFailureW::new(self, 0)
    }
}
#[doc = "See `rrc.sv#L268 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L268>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr_rst0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr_rst0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcsrRst0Spec;
impl crate::RegisterSpec for SfrRrcsrRst0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrcsr_rst0::R`](R) reader structure"]
impl crate::Readable for SfrRrcsrRst0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrcsr_rst0::W`](W) writer structure"]
impl crate::Writable for SfrRrcsrRst0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCSR_RST0 to value 0"]
impl crate::Resettable for SfrRrcsrRst0Spec {}
