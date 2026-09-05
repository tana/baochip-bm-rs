#[doc = "Register `SFR_IO` reader"]
pub type R = crate::R<SfrIoSpec>;
#[doc = "Register `SFR_IO` writer"]
pub type W = crate::W<SfrIoSpec>;
#[doc = "Field `sfr_io` reader - sfr_io read/write control register"]
pub type SfrIoR = crate::FieldReader;
#[doc = "Field `sfr_io` writer - sfr_io read/write control register"]
pub type SfrIoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - sfr_io read/write control register"]
    #[inline(always)]
    pub fn sfr_io(&self) -> SfrIoR {
        SfrIoR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - sfr_io read/write control register"]
    #[inline(always)]
    pub fn sfr_io(&mut self) -> SfrIoW<'_, SfrIoSpec> {
        SfrIoW::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L113 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L113>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_io::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_io::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIoSpec;
impl crate::RegisterSpec for SfrIoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_io::R`](R) reader structure"]
impl crate::Readable for SfrIoSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_io::W`](W) writer structure"]
impl crate::Writable for SfrIoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_IO to value 0"]
impl crate::Resettable for SfrIoSpec {}
