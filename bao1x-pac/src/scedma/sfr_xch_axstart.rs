#[doc = "Register `SFR_XCH_AXSTART` reader"]
pub type R = crate::R<SfrXchAxstartSpec>;
#[doc = "Register `SFR_XCH_AXSTART` writer"]
pub type W = crate::W<SfrXchAxstartSpec>;
#[doc = "Field `xchcr_axstart` reader - xchcr_axstart read/write control register"]
pub type XchcrAxstartR = crate::FieldReader<u32>;
#[doc = "Field `xchcr_axstart` writer - xchcr_axstart read/write control register"]
pub type XchcrAxstartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - xchcr_axstart read/write control register"]
    #[inline(always)]
    pub fn xchcr_axstart(&self) -> XchcrAxstartR {
        XchcrAxstartR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - xchcr_axstart read/write control register"]
    #[inline(always)]
    pub fn xchcr_axstart(&mut self) -> XchcrAxstartW<'_, SfrXchAxstartSpec> {
        XchcrAxstartW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L99 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L99>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_xch_axstart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_xch_axstart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrXchAxstartSpec;
impl crate::RegisterSpec for SfrXchAxstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_xch_axstart::R`](R) reader structure"]
impl crate::Readable for SfrXchAxstartSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_xch_axstart::W`](W) writer structure"]
impl crate::Writable for SfrXchAxstartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_XCH_AXSTART to value 0"]
impl crate::Resettable for SfrXchAxstartSpec {}
