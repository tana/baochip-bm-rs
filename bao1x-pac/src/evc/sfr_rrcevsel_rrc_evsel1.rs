#[doc = "Register `SFR_RRCEVSEL_RRC_EVSEL1` reader"]
pub type R = crate::R<SfrRrcevselRrcEvsel1Spec>;
#[doc = "Register `SFR_RRCEVSEL_RRC_EVSEL1` writer"]
pub type W = crate::W<SfrRrcevselRrcEvsel1Spec>;
#[doc = "Field `rrc_evsel1` reader - rrc_evsel read/write control register"]
pub type RrcEvsel1R = crate::FieldReader<u32>;
#[doc = "Field `rrc_evsel1` writer - rrc_evsel read/write control register"]
pub type RrcEvsel1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - rrc_evsel read/write control register"]
    #[inline(always)]
    pub fn rrc_evsel1(&self) -> RrcEvsel1R {
        RrcEvsel1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - rrc_evsel read/write control register"]
    #[inline(always)]
    pub fn rrc_evsel1(&mut self) -> RrcEvsel1W<'_, SfrRrcevselRrcEvsel1Spec> {
        RrcEvsel1W::new(self, 0)
    }
}
#[doc = "See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcevsel_rrc_evsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcevsel_rrc_evsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcevselRrcEvsel1Spec;
impl crate::RegisterSpec for SfrRrcevselRrcEvsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrcevsel_rrc_evsel1::R`](R) reader structure"]
impl crate::Readable for SfrRrcevselRrcEvsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrcevsel_rrc_evsel1::W`](W) writer structure"]
impl crate::Writable for SfrRrcevselRrcEvsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCEVSEL_RRC_EVSEL1 to value 0"]
impl crate::Resettable for SfrRrcevselRrcEvsel1Spec {}
