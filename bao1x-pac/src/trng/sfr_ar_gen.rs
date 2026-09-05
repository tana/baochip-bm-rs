#[doc = "Register `SFR_AR_GEN` reader"]
pub type R = crate::R<SfrArGenSpec>;
#[doc = "Register `SFR_AR_GEN` writer"]
pub type W = crate::W<SfrArGenSpec>;
#[doc = "Field `sfr_ar_gen` reader - sfr_ar_gen performs action on write of value: 0x55"]
pub type SfrArGenR = crate::FieldReader<u32>;
#[doc = "Field `sfr_ar_gen` writer - sfr_ar_gen performs action on write of value: 0x55"]
pub type SfrArGenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_ar_gen performs action on write of value: 0x55"]
    #[inline(always)]
    pub fn sfr_ar_gen(&self) -> SfrArGenR {
        SfrArGenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_ar_gen performs action on write of value: 0x55"]
    #[inline(always)]
    pub fn sfr_ar_gen(&mut self) -> SfrArGenW<'_, SfrArGenSpec> {
        SfrArGenW::new(self, 0)
    }
}
#[doc = "See `trng.sv#L112 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L112>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_ar_gen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_ar_gen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrArGenSpec;
impl crate::RegisterSpec for SfrArGenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_ar_gen::R`](R) reader structure"]
impl crate::Readable for SfrArGenSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_ar_gen::W`](W) writer structure"]
impl crate::Writable for SfrArGenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_AR_GEN to value 0"]
impl crate::Resettable for SfrArGenSpec {}
