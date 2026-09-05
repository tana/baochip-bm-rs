#[doc = "Register `SFR_XCH_OPT` reader"]
pub type R = crate::R<SfrXchOptSpec>;
#[doc = "Register `SFR_XCH_OPT` writer"]
pub type W = crate::W<SfrXchOptSpec>;
#[doc = "Field `xchcr_opt` reader - xchcr_opt read/write control register"]
pub type XchcrOptR = crate::FieldReader<u16>;
#[doc = "Field `xchcr_opt` writer - xchcr_opt read/write control register"]
pub type XchcrOptW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - xchcr_opt read/write control register"]
    #[inline(always)]
    pub fn xchcr_opt(&self) -> XchcrOptR {
        XchcrOptR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - xchcr_opt read/write control register"]
    #[inline(always)]
    pub fn xchcr_opt(&mut self) -> XchcrOptW<'_, SfrXchOptSpec> {
        XchcrOptW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L98 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ crypto_top/rtl/scedma.sv#L98>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_xch_opt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_xch_opt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrXchOptSpec;
impl crate::RegisterSpec for SfrXchOptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_xch_opt::R`](R) reader structure"]
impl crate::Readable for SfrXchOptSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_xch_opt::W`](W) writer structure"]
impl crate::Writable for SfrXchOptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_XCH_OPT to value 0"]
impl crate::Resettable for SfrXchOptSpec {}
