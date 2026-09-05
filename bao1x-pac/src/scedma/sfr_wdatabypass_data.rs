#[doc = "Register `SFR_WDATABYPASS_DATA` reader"]
pub type R = crate::R<SfrWdatabypassDataSpec>;
#[doc = "Register `SFR_WDATABYPASS_DATA` writer"]
pub type W = crate::W<SfrWdatabypassDataSpec>;
#[doc = "Field `sfr_wdatabypass_data` reader - sfr_wdatabypass_data read/write control register"]
pub type SfrWdatabypassDataR = crate::FieldReader<u32>;
#[doc = "Field `sfr_wdatabypass_data` writer - sfr_wdatabypass_data read/write control register"]
pub type SfrWdatabypassDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_wdatabypass_data read/write control register"]
    #[inline(always)]
    pub fn sfr_wdatabypass_data(&self) -> SfrWdatabypassDataR {
        SfrWdatabypassDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_wdatabypass_data read/write control register"]
    #[inline(always)]
    pub fn sfr_wdatabypass_data(&mut self) -> SfrWdatabypassDataW<'_, SfrWdatabypassDataSpec> {
        SfrWdatabypassDataW::new(self, 0)
    }
}
#[doc = "See `scedma.sv#L118 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules /crypto_top/rtl/scedma.sv#L118>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_wdatabypass_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_wdatabypass_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrWdatabypassDataSpec;
impl crate::RegisterSpec for SfrWdatabypassDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_wdatabypass_data::R`](R) reader structure"]
impl crate::Readable for SfrWdatabypassDataSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_wdatabypass_data::W`](W) writer structure"]
impl crate::Writable for SfrWdatabypassDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_WDATABYPASS_DATA to value 0"]
impl crate::Resettable for SfrWdatabypassDataSpec {}
