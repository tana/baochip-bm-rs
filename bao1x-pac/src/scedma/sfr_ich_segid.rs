#[doc = "Register `SFR_ICH_SEGID` reader"]
pub type R = crate::R<SfrIchSegidSpec>;
#[doc = "Register `SFR_ICH_SEGID` writer"]
pub type W = crate::W<SfrIchSegidSpec>;
#[doc = "Field `sfr_ich_segid` reader - sfr_ich_segid read/write control register"]
pub type SfrIchSegidR = crate::FieldReader<u16>;
#[doc = "Field `sfr_ich_segid` writer - sfr_ich_segid read/write control register"]
pub type SfrIchSegidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_ich_segid read/write control register"]
    #[inline(always)]
    pub fn sfr_ich_segid(&self) -> SfrIchSegidR {
        SfrIchSegidR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_ich_segid read/write control register"]
    #[inline(always)]
    pub fn sfr_ich_segid(&mut self) -> SfrIchSegidW<'_, SfrIchSegidSpec> {
        SfrIchSegidW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L112 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L112>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ich_segid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ich_segid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIchSegidSpec;
impl crate::RegisterSpec for SfrIchSegidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ich_segid::R`](R) reader structure"]
impl crate::Readable for SfrIchSegidSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ich_segid::W`](W) writer structure"]
impl crate::Writable for SfrIchSegidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ICH_SEGID to value 0"]
impl crate::Resettable for SfrIchSegidSpec {}
