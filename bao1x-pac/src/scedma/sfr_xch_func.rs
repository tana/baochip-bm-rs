#[doc = "Register `SFR_XCH_FUNC` reader"]
pub type R = crate::R<SfrXchFuncSpec>;
#[doc = "Register `SFR_XCH_FUNC` writer"]
pub type W = crate::W<SfrXchFuncSpec>;
#[doc = "Field `xchcr_func` reader - xchcr_func read/write control register"]
pub type XchcrFuncR = crate::BitReader;
#[doc = "Field `xchcr_func` writer - xchcr_func read/write control register"]
pub type XchcrFuncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - xchcr_func read/write control register"]
    #[inline(always)]
    pub fn xchcr_func(&self) -> XchcrFuncR {
        XchcrFuncR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - xchcr_func read/write control register"]
    #[inline(always)]
    pub fn xchcr_func(&mut self) -> XchcrFuncW<'_, SfrXchFuncSpec> {
        XchcrFuncW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L97 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L97>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_xch_func::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_xch_func::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrXchFuncSpec;
impl crate::RegisterSpec for SfrXchFuncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_xch_func::R`](R) reader structure"]
impl crate::Readable for SfrXchFuncSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_xch_func::W`](W) writer structure"]
impl crate::Writable for SfrXchFuncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_XCH_FUNC to value 0"]
impl crate::Resettable for SfrXchFuncSpec {}
