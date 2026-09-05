#[doc = "Register `SFR_CGULP` reader"]
pub type R = crate::R<SfrCgulpSpec>;
#[doc = "Register `SFR_CGULP` writer"]
pub type W = crate::W<SfrCgulpSpec>;
#[doc = "Field `sfr_cgulp` reader - sfr_cgulp read/write control register"]
pub type SfrCgulpR = crate::FieldReader<u16>;
#[doc = "Field `sfr_cgulp` writer - sfr_cgulp read/write control register"]
pub type SfrCgulpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_cgulp read/write control register"]
    #[inline(always)]
    pub fn sfr_cgulp(&self) -> SfrCgulpR {
        SfrCgulpR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_cgulp read/write control register"]
    #[inline(always)]
    pub fn sfr_cgulp(&mut self) -> SfrCgulpW<'_, SfrCgulpSpec> {
        SfrCgulpW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L769 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L769>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgulp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgulp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgulpSpec;
impl crate::RegisterSpec for SfrCgulpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgulp::R`](R) reader structure"]
impl crate::Readable for SfrCgulpSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgulp::W`](W) writer structure"]
impl crate::Writable for SfrCgulpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGULP to value 0"]
impl crate::Resettable for SfrCgulpSpec {}
