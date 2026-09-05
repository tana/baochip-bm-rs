#[doc = "Register `SFR_RRCSR` reader"]
pub type R = crate::R<SfrRrcsrSpec>;
#[doc = "Register `SFR_RRCSR` writer"]
pub type W = crate::W<SfrRrcsrSpec>;
#[doc = "Field `sfr_rrcsr` reader - sfr_rrcsr read only status register"]
pub type SfrRrcsrR = crate::FieldReader<u16>;
#[doc = "Field `sfr_rrcsr` writer - sfr_rrcsr read only status register"]
pub type SfrRrcsrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - sfr_rrcsr read only status register"]
    #[inline(always)]
    pub fn sfr_rrcsr(&self) -> SfrRrcsrR {
        SfrRrcsrR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - sfr_rrcsr read only status register"]
    #[inline(always)]
    pub fn sfr_rrcsr(&mut self) -> SfrRrcsrW<'_, SfrRrcsrSpec> {
        SfrRrcsrW::new(self, 0)
    }
}
#[doc = "See `rrc.sv#L263 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L263>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcsrSpec;
impl crate::RegisterSpec for SfrRrcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrcsr::R`](R) reader structure"]
impl crate::Readable for SfrRrcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrcsr::W`](W) writer structure"]
impl crate::Writable for SfrRrcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCSR to value 0"]
impl crate::Resettable for SfrRrcsrSpec {}
