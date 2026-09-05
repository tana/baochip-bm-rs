#[doc = "Register `SFR_CM7EVEN` reader"]
pub type R = crate::R<SfrCm7evenSpec>;
#[doc = "Register `SFR_CM7EVEN` writer"]
pub type W = crate::W<SfrCm7evenSpec>;
#[doc = "Field `cm7even` reader - cm7even read/write control register"]
pub type Cm7evenR = crate::FieldReader;
#[doc = "Field `cm7even` writer - cm7even read/write control register"]
pub type Cm7evenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - cm7even read/write control register"]
    #[inline(always)]
    pub fn cm7even(&self) -> Cm7evenR {
        Cm7evenR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - cm7even read/write control register"]
    #[inline(always)]
    pub fn cm7even(&mut self) -> Cm7evenW<'_, SfrCm7evenSpec> {
        Cm7evenW::new(self, 0)
    }
}
#[doc = "See `evc.sv#L141 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/sy sctrl/rtl/evc.sv#L141>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cm7even::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cm7even::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCm7evenSpec;
impl crate::RegisterSpec for SfrCm7evenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cm7even::R`](R) reader structure"]
impl crate::Readable for SfrCm7evenSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cm7even::W`](W) writer structure"]
impl crate::Writable for SfrCm7evenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CM7EVEN to value 0"]
impl crate::Resettable for SfrCm7evenSpec {}
