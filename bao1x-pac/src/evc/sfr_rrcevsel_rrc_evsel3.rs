#[doc = "Register `SFR_RRCEVSEL_RRC_EVSEL3` reader"]
pub type R = crate::R<SfrRrcevselRrcEvsel3Spec>;
#[doc = "Register `SFR_RRCEVSEL_RRC_EVSEL3` writer"]
pub type W = crate::W<SfrRrcevselRrcEvsel3Spec>;
#[doc = "Field `rrc_evsel3` reader - rrc_evsel read/write control register"]
pub type RrcEvsel3R = crate::FieldReader<u32>;
#[doc = "Field `rrc_evsel3` writer - rrc_evsel read/write control register"]
pub type RrcEvsel3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - rrc_evsel read/write control register"]
    #[inline(always)]
    pub fn rrc_evsel3(&self) -> RrcEvsel3R {
        RrcEvsel3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - rrc_evsel read/write control register"]
    #[inline(always)]
    pub fn rrc_evsel3(&mut self) -> RrcEvsel3W<'_, SfrRrcevselRrcEvsel3Spec> {
        RrcEvsel3W::new(self, 0)
    }
}
#[doc = "See `evc.sv#L153 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L153>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcevsel_rrc_evsel3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcevsel_rrc_evsel3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcevselRrcEvsel3Spec;
impl crate::RegisterSpec for SfrRrcevselRrcEvsel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrcevsel_rrc_evsel3::R`](R) reader structure"]
impl crate::Readable for SfrRrcevselRrcEvsel3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrcevsel_rrc_evsel3::W`](W) writer structure"]
impl crate::Writable for SfrRrcevselRrcEvsel3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCEVSEL_RRC_EVSEL3 to value 0"]
impl crate::Resettable for SfrRrcevselRrcEvsel3Spec {}
