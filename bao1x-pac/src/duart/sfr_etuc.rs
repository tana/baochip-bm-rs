#[doc = "Register `SFR_ETUC` reader"]
pub type R = crate::R<SfrEtucSpec>;
#[doc = "Register `SFR_ETUC` writer"]
pub type W = crate::W<SfrEtucSpec>;
#[doc = "Field `sfr_etuc` reader - sfr_etuc read/write control register"]
pub type SfrEtucR = crate::FieldReader<u16>;
#[doc = "Field `sfr_etuc` writer - sfr_etuc read/write control register"]
pub type SfrEtucW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_etuc read/write control register"]
    #[inline(always)]
    pub fn sfr_etuc(&self) -> SfrEtucR {
        SfrEtucR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_etuc read/write control register"]
    #[inline(always)]
    pub fn sfr_etuc(&mut self) -> SfrEtucW<'_, SfrEtucSpec> {
        SfrEtucW::new(self, 0)
    }
}
#[doc = "See `duart.sv#L45 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L45>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_etuc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_etuc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrEtucSpec;
impl crate::RegisterSpec for SfrEtucSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_etuc::R`](R) reader structure"]
impl crate::Readable for SfrEtucSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_etuc::W`](W) writer structure"]
impl crate::Writable for SfrEtucSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_ETUC to value 0"]
impl crate::Resettable for SfrEtucSpec {}
