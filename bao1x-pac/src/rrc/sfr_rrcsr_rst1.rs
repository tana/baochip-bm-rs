#[doc = "Register `SFR_RRCSR_RST1` reader"]
pub type R = crate::R<SfrRrcsrRst1Spec>;
#[doc = "Register `SFR_RRCSR_RST1` writer"]
pub type W = crate::W<SfrRrcsrRst1Spec>;
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
    pub fn trc_reset_failure(&mut self) -> TrcResetFailureW<'_, SfrRrcsrRst1Spec> {
        TrcResetFailureW::new(self, 0)
    }
}
#[doc = "See `rrc.sv#L269 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L269>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr_rst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr_rst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcsrRst1Spec;
impl crate::RegisterSpec for SfrRrcsrRst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrcsr_rst1::R`](R) reader structure"]
impl crate::Readable for SfrRrcsrRst1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrcsr_rst1::W`](W) writer structure"]
impl crate::Writable for SfrRrcsrRst1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCSR_RST1 to value 0"]
impl crate::Resettable for SfrRrcsrRst1Spec {}
