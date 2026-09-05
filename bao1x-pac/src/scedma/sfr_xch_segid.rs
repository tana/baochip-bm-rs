#[doc = "Register `SFR_XCH_SEGID` reader"]
pub type R = crate::R<SfrXchSegidSpec>;
#[doc = "Register `SFR_XCH_SEGID` writer"]
pub type W = crate::W<SfrXchSegidSpec>;
#[doc = "Field `xchcr_segid` reader - xchcr_segid read/write control register"]
pub type XchcrSegidR = crate::FieldReader;
#[doc = "Field `xchcr_segid` writer - xchcr_segid read/write control register"]
pub type XchcrSegidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - xchcr_segid read/write control register"]
    #[inline(always)]
    pub fn xchcr_segid(&self) -> XchcrSegidR {
        XchcrSegidR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - xchcr_segid read/write control register"]
    #[inline(always)]
    pub fn xchcr_segid(&mut self) -> XchcrSegidW<'_, SfrXchSegidSpec> {
        XchcrSegidW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L100 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L100>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_xch_segid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_xch_segid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrXchSegidSpec;
impl crate::RegisterSpec for SfrXchSegidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_xch_segid::R`](R) reader structure"]
impl crate::Readable for SfrXchSegidSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_xch_segid::W`](W) writer structure"]
impl crate::Writable for SfrXchSegidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_XCH_SEGID to value 0"]
impl crate::Resettable for SfrXchSegidSpec {}
