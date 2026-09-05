#[doc = "Register `SFR_SRAMERR` reader"]
pub type R = crate::R<SfrSramerrSpec>;
#[doc = "Register `SFR_SRAMERR` writer"]
pub type W = crate::W<SfrSramerrSpec>;
#[doc = "Field `srambankerr` reader - srambankerr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type SrambankerrR = crate::FieldReader;
#[doc = "Field `srambankerr` writer - srambankerr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type SrambankerrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - srambankerr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn srambankerr(&self) -> SrambankerrR {
        SrambankerrR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - srambankerr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn srambankerr(&mut self) -> SrambankerrW<'_, SfrSramerrSpec> {
        SrambankerrW::new(self, 0)
    }
}
#[doc = "See `coresub_sramtrm.sv#L60 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L60>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sramerr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sramerr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSramerrSpec;
impl crate::RegisterSpec for SfrSramerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sramerr::R`](R) reader structure"]
impl crate::Readable for SfrSramerrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sramerr::W`](W) writer structure"]
impl crate::Writable for SfrSramerrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SRAMERR to value 0"]
impl crate::Resettable for SfrSramerrSpec {}
