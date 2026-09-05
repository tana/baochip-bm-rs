#[doc = "Register `SFR_RRCEVSEL_RRC_EVSEL2` reader"]
pub type R = crate::R<SfrRrcevselRrcEvsel2Spec>;
#[doc = "Register `SFR_RRCEVSEL_RRC_EVSEL2` writer"]
pub type W = crate::W<SfrRrcevselRrcEvsel2Spec>;
#[doc = "Field `rrc_evsel2` reader - rrc_evsel read/write control register"]
pub type RrcEvsel2R = crate::FieldReader<u32>;
#[doc = "Field `rrc_evsel2` writer - rrc_evsel read/write control register"]
pub type RrcEvsel2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - rrc_evsel read/write control register"]
    #[inline(always)]
    pub fn rrc_evsel2(&self) -> RrcEvsel2R {
        RrcEvsel2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - rrc_evsel read/write control register"]
    #[inline(always)]
    pub fn rrc_evsel2(&mut self) -> RrcEvsel2W<'_, SfrRrcevselRrcEvsel2Spec> {
        RrcEvsel2W::new(self, 0)
    }
}
#[doc = "See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcevsel_rrc_evsel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcevsel_rrc_evsel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcevselRrcEvsel2Spec;
impl crate::RegisterSpec for SfrRrcevselRrcEvsel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrcevsel_rrc_evsel2::R`](R) reader structure"]
impl crate::Readable for SfrRrcevselRrcEvsel2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrcevsel_rrc_evsel2::W`](W) writer structure"]
impl crate::Writable for SfrRrcevselRrcEvsel2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCEVSEL_RRC_EVSEL2 to value 0"]
impl crate::Resettable for SfrRrcevselRrcEvsel2Spec {}
