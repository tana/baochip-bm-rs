#[doc = "Register `SFR_VEXRAM` reader"]
pub type R = crate::R<SfrVexramSpec>;
#[doc = "Register `SFR_VEXRAM` writer"]
pub type W = crate::W<SfrVexramSpec>;
#[doc = "Field `sfr_vexram` reader - sfr_vexram read/write control register"]
pub type SfrVexramR = crate::FieldReader;
#[doc = "Field `sfr_vexram` writer - sfr_vexram read/write control register"]
pub type SfrVexramW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - sfr_vexram read/write control register"]
    #[inline(always)]
    pub fn sfr_vexram(&self) -> SfrVexramR {
        SfrVexramR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - sfr_vexram read/write control register"]
    #[inline(always)]
    pub fn sfr_vexram(&mut self) -> SfrVexramW<'_, SfrVexramSpec> {
        SfrVexramW::new(self, 0)
    }
}
#[doc = "See `coresub_sramtrm.sv#L59 <https://github.com/baochip/baochip-1x/blob/main/rtl /modules/core/rtl/coresub_sramtrm.sv#L59>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_vexram::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_vexram::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrVexramSpec;
impl crate::RegisterSpec for SfrVexramSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_vexram::R`](R) reader structure"]
impl crate::Readable for SfrVexramSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_vexram::W`](W) writer structure"]
impl crate::Writable for SfrVexramSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_VEXRAM to value 0"]
impl crate::Resettable for SfrVexramSpec {}
