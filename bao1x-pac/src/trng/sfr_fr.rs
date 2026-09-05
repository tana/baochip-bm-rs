#[doc = "Register `SFR_FR` reader"]
pub type R = crate::R<SfrFrSpec>;
#[doc = "Register `SFR_FR` writer"]
pub type W = crate::W<SfrFrSpec>;
#[doc = "Field `sfr_fr` reader - sfr_fr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type SfrFrR = crate::FieldReader;
#[doc = "Field `sfr_fr` writer - sfr_fr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
pub type SfrFrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - sfr_fr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn sfr_fr(&self) -> SfrFrR {
        SfrFrR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - sfr_fr flag register. `1` means event happened, write back `1` in respective bit position to clear the flag"]
    #[inline(always)]
    pub fn sfr_fr(&mut self) -> SfrFrW<'_, SfrFrSpec> {
        SfrFrW::new(self, 0)
    }
}
#[doc = "See `trng.sv#L115 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L115>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_fr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_fr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrFrSpec;
impl crate::RegisterSpec for SfrFrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_fr::R`](R) reader structure"]
impl crate::Readable for SfrFrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_fr::W`](W) writer structure"]
impl crate::Writable for SfrFrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_FR to value 0"]
impl crate::Resettable for SfrFrSpec {}
