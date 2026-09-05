#[doc = "Register `SFR_SR` reader"]
pub type R = crate::R<SfrSrSpec>;
#[doc = "Register `SFR_SR` writer"]
pub type W = crate::W<SfrSrSpec>;
#[doc = "Field `sfr_sr` reader - sfr_sr read only status register"]
pub type SfrSrR = crate::BitReader;
#[doc = "Field `sfr_sr` writer - sfr_sr read only status register"]
pub type SfrSrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - sfr_sr read only status register"]
    #[inline(always)]
    pub fn sfr_sr(&self) -> SfrSrR {
        SfrSrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sfr_sr read only status register"]
    #[inline(always)]
    pub fn sfr_sr(&mut self) -> SfrSrW<'_, SfrSrSpec> {
        SfrSrW::new(self, 0)
    }
}
#[doc = "See `duart.sv#L44 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c ore/rtl/duart.sv#L44>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSrSpec;
impl crate::RegisterSpec for SfrSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sr::R`](R) reader structure"]
impl crate::Readable for SfrSrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sr::W`](W) writer structure"]
impl crate::Writable for SfrSrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SR to value 0"]
impl crate::Resettable for SfrSrSpec {}
