#[doc = "Register `SFR_ICH_OPT` reader"]
pub type R = crate::R<SfrIchOptSpec>;
#[doc = "Register `SFR_ICH_OPT` writer"]
pub type W = crate::W<SfrIchOptSpec>;
#[doc = "Field `sfr_ich_opt` reader - sfr_ich_opt read/write control register"]
pub type SfrIchOptR = crate::FieldReader;
#[doc = "Field `sfr_ich_opt` writer - sfr_ich_opt read/write control register"]
pub type SfrIchOptW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - sfr_ich_opt read/write control register"]
    #[inline(always)]
    pub fn sfr_ich_opt(&self) -> SfrIchOptR {
        SfrIchOptR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - sfr_ich_opt read/write control register"]
    #[inline(always)]
    pub fn sfr_ich_opt(&mut self) -> SfrIchOptW<'_, SfrIchOptSpec> {
        SfrIchOptW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L111 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L111>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ich_opt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ich_opt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIchOptSpec;
impl crate::RegisterSpec for SfrIchOptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ich_opt::R`](R) reader structure"]
impl crate::Readable for SfrIchOptSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ich_opt::W`](W) writer structure"]
impl crate::Writable for SfrIchOptSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ICH_OPT to value 0"]
impl crate::Resettable for SfrIchOptSpec {}
