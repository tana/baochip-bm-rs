#[doc = "Register `SFR_CRANA` reader"]
pub type R = crate::R<SfrCranaSpec>;
#[doc = "Register `SFR_CRANA` writer"]
pub type W = crate::W<SfrCranaSpec>;
#[doc = "Field `sfr_crana` reader - sfr_crana read/write control register"]
pub type SfrCranaR = crate::FieldReader<u16>;
#[doc = "Field `sfr_crana` writer - sfr_crana read/write control register"]
pub type SfrCranaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_crana read/write control register"]
    #[inline(always)]
    pub fn sfr_crana(&self) -> SfrCranaR {
        SfrCranaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_crana read/write control register"]
    #[inline(always)]
    pub fn sfr_crana(&mut self) -> SfrCranaW<'_, SfrCranaSpec> {
        SfrCranaW::new(self, 0)
    }
}
#[doc = "See `trng.sv#L106 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/c rypto_top/rtl/trng.sv#L106>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_crana::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_crana::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCranaSpec;
impl crate::RegisterSpec for SfrCranaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_crana::R`](R) reader structure"]
impl crate::Readable for SfrCranaSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_crana::W`](W) writer structure"]
impl crate::Writable for SfrCranaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CRANA to value 0"]
impl crate::Resettable for SfrCranaSpec {}
