#[doc = "Register `SFR_RRCFD` reader"]
pub type R = crate::R<SfrRrcfdSpec>;
#[doc = "Register `SFR_RRCFD` writer"]
pub type W = crate::W<SfrRrcfdSpec>;
#[doc = "Field `sfr_rrcfd` reader - sfr_rrcfd read/write control register"]
pub type SfrRrcfdR = crate::FieldReader;
#[doc = "Field `sfr_rrcfd` writer - sfr_rrcfd read/write control register"]
pub type SfrRrcfdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - sfr_rrcfd read/write control register"]
    #[inline(always)]
    pub fn sfr_rrcfd(&self) -> SfrRrcfdR {
        SfrRrcfdR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - sfr_rrcfd read/write control register"]
    #[inline(always)]
    pub fn sfr_rrcfd(&mut self) -> SfrRrcfdW<'_, SfrRrcfdSpec> {
        SfrRrcfdW::new(self, 0)
    }
}
#[doc = "See `rrc.sv#L262 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L262>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcfd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcfd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcfdSpec;
impl crate::RegisterSpec for SfrRrcfdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrcfd::R`](R) reader structure"]
impl crate::Readable for SfrRrcfdSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrcfd::W`](W) writer structure"]
impl crate::Writable for SfrRrcfdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCFD to value 0"]
impl crate::Resettable for SfrRrcfdSpec {}
