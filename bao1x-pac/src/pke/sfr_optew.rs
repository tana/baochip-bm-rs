#[doc = "Register `SFR_OPTEW` reader"]
pub type R = crate::R<SfrOptewSpec>;
#[doc = "Register `SFR_OPTEW` writer"]
pub type W = crate::W<SfrOptewSpec>;
#[doc = "Field `sfr_optew` reader - sfr_optew read/write control register"]
pub type SfrOptewR = crate::FieldReader<u16>;
#[doc = "Field `sfr_optew` writer - sfr_optew read/write control register"]
pub type SfrOptewW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - sfr_optew read/write control register"]
    #[inline(always)]
    pub fn sfr_optew(&self) -> SfrOptewR {
        SfrOptewR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - sfr_optew read/write control register"]
    #[inline(always)]
    pub fn sfr_optew(&mut self) -> SfrOptewW<'_, SfrOptewSpec> {
        SfrOptewW::new(self, 0)
    }
}
#[doc = "See `pke.sv#L301 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L301>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_optew::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_optew::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrOptewSpec;
impl crate::RegisterSpec for SfrOptewSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_optew::R`](R) reader structure"]
impl crate::Readable for SfrOptewSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_optew::W`](W) writer structure"]
impl crate::Writable for SfrOptewSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_OPTEW to value 0"]
impl crate::Resettable for SfrOptewSpec {}
