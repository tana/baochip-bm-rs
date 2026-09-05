#[doc = "Register `SFR_RRCSR_RD1` reader"]
pub type R = crate::R<SfrRrcsrRd1Spec>;
#[doc = "Register `SFR_RRCSR_RD1` writer"]
pub type W = crate::W<SfrRrcsrRd1Spec>;
#[doc = "Field `trc_fourth_read_failure` reader - trc_fourth_read_failure read only status register"]
pub type TrcFourthReadFailureR = crate::FieldReader<u32>;
#[doc = "Field `trc_fourth_read_failure` writer - trc_fourth_read_failure read only status register"]
pub type TrcFourthReadFailureW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - trc_fourth_read_failure read only status register"]
    #[inline(always)]
    pub fn trc_fourth_read_failure(&self) -> TrcFourthReadFailureR {
        TrcFourthReadFailureR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - trc_fourth_read_failure read only status register"]
    #[inline(always)]
    pub fn trc_fourth_read_failure(&mut self) -> TrcFourthReadFailureW<'_, SfrRrcsrRd1Spec> {
        TrcFourthReadFailureW::new(self, 0)
    }
}
#[doc = "See `rrc.sv#L271 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L271>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr_rd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr_rd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcsrRd1Spec;
impl crate::RegisterSpec for SfrRrcsrRd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrcsr_rd1::R`](R) reader structure"]
impl crate::Readable for SfrRrcsrRd1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrcsr_rd1::W`](W) writer structure"]
impl crate::Writable for SfrRrcsrRd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCSR_RD1 to value 0"]
impl crate::Resettable for SfrRrcsrRd1Spec {}
