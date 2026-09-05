#[doc = "Register `SFR_RAMSEC` reader"]
pub type R = crate::R<SfrRamsecSpec>;
#[doc = "Register `SFR_RAMSEC` writer"]
pub type W = crate::W<SfrRamsecSpec>;
#[doc = "Field `ramsec` reader - ramsec read/write control register"]
pub type RamsecR = crate::FieldReader;
#[doc = "Field `ramsec` writer - ramsec read/write control register"]
pub type RamsecW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ramsec read/write control register"]
    #[inline(always)]
    pub fn ramsec(&self) -> RamsecR {
        RamsecR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ramsec read/write control register"]
    #[inline(always)]
    pub fn ramsec(&mut self) -> RamsecW<'_, SfrRamsecSpec> {
        RamsecW::new(self, 0)
    }
}
#[doc = "See `coresub_sramtrm.sv#L61 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L61>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ramsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ramsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRamsecSpec;
impl crate::RegisterSpec for SfrRamsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ramsec::R`](R) reader structure"]
impl crate::Readable for SfrRamsecSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ramsec::W`](W) writer structure"]
impl crate::Writable for SfrRamsecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RAMSEC to value 0"]
impl crate::Resettable for SfrRamsecSpec {}
