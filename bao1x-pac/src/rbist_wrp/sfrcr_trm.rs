#[doc = "Register `SFRCR_TRM` reader"]
pub type R = crate::R<SfrcrTrmSpec>;
#[doc = "Register `SFRCR_TRM` writer"]
pub type W = crate::W<SfrcrTrmSpec>;
#[doc = "Field `sfrcr_trm` reader - sfrcr_trm read/write control register"]
pub type SfrcrTrmR = crate::FieldReader<u32>;
#[doc = "Field `sfrcr_trm` writer - sfrcr_trm read/write control register"]
pub type SfrcrTrmW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - sfrcr_trm read/write control register"]
    #[inline(always)]
    pub fn sfrcr_trm(&self) -> SfrcrTrmR {
        SfrcrTrmR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - sfrcr_trm read/write control register"]
    #[inline(always)]
    pub fn sfrcr_trm(&mut self) -> SfrcrTrmW<'_, SfrcrTrmSpec> {
        SfrcrTrmW::new(self, 0)
    }
}
#[doc = "See `rbist_wrp.sv#L174 <https://github.com/baochip/baochip-1x/blob/main/rtl/modu les/rbist/rtl/rbist_wrp.sv#L174>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfrcr_trm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfrcr_trm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrcrTrmSpec;
impl crate::RegisterSpec for SfrcrTrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfrcr_trm::R`](R) reader structure"]
impl crate::Readable for SfrcrTrmSpec {}
#[doc = "`write(|w| ..)` method takes [`sfrcr_trm::W`](W) writer structure"]
impl crate::Writable for SfrcrTrmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFRCR_TRM to value 0"]
impl crate::Resettable for SfrcrTrmSpec {}
