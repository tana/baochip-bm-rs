#[doc = "Register `SFRSR_TRM` reader"]
pub type R = crate::R<SfrsrTrmSpec>;
#[doc = "Register `SFRSR_TRM` writer"]
pub type W = crate::W<SfrsrTrmSpec>;
#[doc = "Field `sfrsr_trm` reader - sfrsr_trm read only status register"]
pub type SfrsrTrmR = crate::FieldReader<u32>;
#[doc = "Field `sfrsr_trm` writer - sfrsr_trm read only status register"]
pub type SfrsrTrmW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - sfrsr_trm read only status register"]
    #[inline(always)]
    pub fn sfrsr_trm(&self) -> SfrsrTrmR {
        SfrsrTrmR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - sfrsr_trm read only status register"]
    #[inline(always)]
    pub fn sfrsr_trm(&mut self) -> SfrsrTrmW<'_, SfrsrTrmSpec> {
        SfrsrTrmW::new(self, 0)
    }
}
#[doc = "See `rbist_wrp.sv#L175 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/rbist/rtl/rbist_wrp.sv#L175>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrsr_trm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrsr_trm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrsrTrmSpec;
impl crate::RegisterSpec for SfrsrTrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfrsr_trm::R`](R) reader structure"]
impl crate::Readable for SfrsrTrmSpec {}
#[doc = "`write(|w| ..)` method takes [`sfrsr_trm::W`](W) writer structure"]
impl crate::Writable for SfrsrTrmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRSR_TRM to value 0"]
impl crate::Resettable for SfrsrTrmSpec {}
