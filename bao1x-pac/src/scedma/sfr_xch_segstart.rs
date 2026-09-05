#[doc = "Register `SFR_XCH_SEGSTART` reader"]
pub type R = crate::R<SfrXchSegstartSpec>;
#[doc = "Register `SFR_XCH_SEGSTART` writer"]
pub type W = crate::W<SfrXchSegstartSpec>;
#[doc = "Field `xchcr_segstart` reader - xchcr_segstart read/write control register"]
pub type XchcrSegstartR = crate::FieldReader<u16>;
#[doc = "Field `xchcr_segstart` writer - xchcr_segstart read/write control register"]
pub type XchcrSegstartW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - xchcr_segstart read/write control register"]
    #[inline(always)]
    pub fn xchcr_segstart(&self) -> XchcrSegstartR {
        XchcrSegstartR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - xchcr_segstart read/write control register"]
    #[inline(always)]
    pub fn xchcr_segstart(&mut self) -> XchcrSegstartW<'_, SfrXchSegstartSpec> {
        XchcrSegstartW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L101 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L101>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_xch_segstart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_xch_segstart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrXchSegstartSpec;
impl crate::RegisterSpec for SfrXchSegstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_xch_segstart::R`](R) reader structure"]
impl crate::Readable for SfrXchSegstartSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_xch_segstart::W`](W) writer structure"]
impl crate::Writable for SfrXchSegstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_XCH_SEGSTART to value 0"]
impl crate::Resettable for SfrXchSegstartSpec {}
