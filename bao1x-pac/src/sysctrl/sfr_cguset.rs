#[doc = "Register `SFR_CGUSET` reader"]
pub type R = crate::R<SfrCgusetSpec>;
#[doc = "Register `SFR_CGUSET` writer"]
pub type W = crate::W<SfrCgusetSpec>;
#[doc = "Field `sfr_cguset` reader - sfr_cguset performs action on write of value: 0x32"]
pub type SfrCgusetR = crate::FieldReader<u32>;
#[doc = "Field `sfr_cguset` writer - sfr_cguset performs action on write of value: 0x32"]
pub type SfrCgusetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_cguset performs action on write of value: 0x32"]
    #[inline(always)]
    pub fn sfr_cguset(&self) -> SfrCgusetR {
        SfrCgusetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_cguset performs action on write of value: 0x32"]
    #[inline(always)]
    pub fn sfr_cguset(&mut self) -> SfrCgusetW<'_, SfrCgusetSpec> {
        SfrCgusetW::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L781 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L781>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cguset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cguset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgusetSpec;
impl crate::RegisterSpec for SfrCgusetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cguset::R`](R) reader structure"]
impl crate::Readable for SfrCgusetSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cguset::W`](W) writer structure"]
impl crate::Writable for SfrCgusetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUSET to value 0"]
impl crate::Resettable for SfrCgusetSpec {}
