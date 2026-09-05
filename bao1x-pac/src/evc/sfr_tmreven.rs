#[doc = "Register `SFR_TMREVEN` reader"]
pub type R = crate::R<SfrTmrevenSpec>;
#[doc = "Register `SFR_TMREVEN` writer"]
pub type W = crate::W<SfrTmrevenSpec>;
#[doc = "Field `sfr_tmreven` reader - sfr_tmreven read/write control register"]
pub type SfrTmrevenR = crate::FieldReader;
#[doc = "Field `sfr_tmreven` writer - sfr_tmreven read/write control register"]
pub type SfrTmrevenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - sfr_tmreven read/write control register"]
    #[inline(always)]
    pub fn sfr_tmreven(&self) -> SfrTmrevenR {
        SfrTmrevenR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - sfr_tmreven read/write control register"]
    #[inline(always)]
    pub fn sfr_tmreven(&mut self) -> SfrTmrevenW<'_, SfrTmrevenSpec> {
        SfrTmrevenW::new(self, 0)
    }
}
#[doc = "See `evc.sv#L145 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L145>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_tmreven::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_tmreven::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrTmrevenSpec;
impl crate::RegisterSpec for SfrTmrevenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_tmreven::R`](R) reader structure"]
impl crate::Readable for SfrTmrevenSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_tmreven::W`](W) writer structure"]
impl crate::Writable for SfrTmrevenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_TMREVEN to value 0"]
impl crate::Resettable for SfrTmrevenSpec {}
