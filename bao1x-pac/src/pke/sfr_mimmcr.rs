#[doc = "Register `SFR_MIMMCR` reader"]
pub type R = crate::R<SfrMimmcrSpec>;
#[doc = "Register `SFR_MIMMCR` writer"]
pub type W = crate::W<SfrMimmcrSpec>;
#[doc = "Field `sfr_mimmcr` reader - sfr_mimmcr read/write control register"]
pub type SfrMimmcrR = crate::FieldReader<u16>;
#[doc = "Field `sfr_mimmcr` writer - sfr_mimmcr read/write control register"]
pub type SfrMimmcrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - sfr_mimmcr read/write control register"]
    #[inline(always)]
    pub fn sfr_mimmcr(&self) -> SfrMimmcrR {
        SfrMimmcrR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - sfr_mimmcr read/write control register"]
    #[inline(always)]
    pub fn sfr_mimmcr(&mut self) -> SfrMimmcrW<'_, SfrMimmcrSpec> {
        SfrMimmcrW::new(self, 0)
    }
}
#[doc = "See `pke.sv#L306 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/pke.sv#L306>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_mimmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_mimmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrMimmcrSpec;
impl crate::RegisterSpec for SfrMimmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_mimmcr::R`](R) reader structure"]
impl crate::Readable for SfrMimmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_mimmcr::W`](W) writer structure"]
impl crate::Writable for SfrMimmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_MIMMCR to value 0"]
impl crate::Resettable for SfrMimmcrSpec {}
