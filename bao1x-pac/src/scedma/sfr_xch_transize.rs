#[doc = "Register `SFR_XCH_TRANSIZE` reader"]
pub type R = crate::R<SfrXchTransizeSpec>;
#[doc = "Register `SFR_XCH_TRANSIZE` writer"]
pub type W = crate::W<SfrXchTransizeSpec>;
#[doc = "Field `xchcr_transize` reader - xchcr_transize read/write control register"]
pub type XchcrTransizeR = crate::FieldReader<u32>;
#[doc = "Field `xchcr_transize` writer - xchcr_transize read/write control register"]
pub type XchcrTransizeW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - xchcr_transize read/write control register"]
    #[inline(always)]
    pub fn xchcr_transize(&self) -> XchcrTransizeR {
        XchcrTransizeR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - xchcr_transize read/write control register"]
    #[inline(always)]
    pub fn xchcr_transize(&mut self) -> XchcrTransizeW<'_, SfrXchTransizeSpec> {
        XchcrTransizeW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L102 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L102>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_xch_transize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_xch_transize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrXchTransizeSpec;
impl crate::RegisterSpec for SfrXchTransizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_xch_transize::R`](R) reader structure"]
impl crate::Readable for SfrXchTransizeSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_xch_transize::W`](W) writer structure"]
impl crate::Writable for SfrXchTransizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_XCH_TRANSIZE to value 0"]
impl crate::Resettable for SfrXchTransizeSpec {}
