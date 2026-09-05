#[doc = "Register `SFR_RRCAR` reader"]
pub type R = crate::R<SfrRrcarSpec>;
#[doc = "Register `SFR_RRCAR` writer"]
pub type W = crate::W<SfrRrcarSpec>;
#[doc = "Field `sfr_rrcar` reader - sfr_rrcar performs action on write of value: 0x2468"]
pub type SfrRrcarR = crate::FieldReader<u32>;
#[doc = "Field `sfr_rrcar` writer - sfr_rrcar performs action on write of value: 0x2468"]
pub type SfrRrcarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_rrcar performs action on write of value: 0x2468"]
    #[inline(always)]
    pub fn sfr_rrcar(&self) -> SfrRrcarR {
        SfrRrcarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_rrcar performs action on write of value: 0x2468"]
    #[inline(always)]
    pub fn sfr_rrcar(&mut self) -> SfrRrcarW<'_, SfrRrcarSpec> {
        SfrRrcarW::new(self, 0)
    }
}
#[doc = "See `rrc.sv#L273 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/rr c/rtl/rrc.sv#L273>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rrcar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rrcar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRrcarSpec;
impl crate::RegisterSpec for SfrRrcarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rrcar::R`](R) reader structure"]
impl crate::Readable for SfrRrcarSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rrcar::W`](W) writer structure"]
impl crate::Writable for SfrRrcarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RRCAR to value 0"]
impl crate::Resettable for SfrRrcarSpec {}
