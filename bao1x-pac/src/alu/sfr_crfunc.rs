#[doc = "Register `SFR_CRFUNC` reader"]
pub type R = crate::R<SfrCrfuncSpec>;
#[doc = "Register `SFR_CRFUNC` writer"]
pub type W = crate::W<SfrCrfuncSpec>;
#[doc = "Field `sfr_crfunc` reader - sfr_crfunc read/write control register"]
pub type SfrCrfuncR = crate::FieldReader;
#[doc = "Field `sfr_crfunc` writer - sfr_crfunc read/write control register"]
pub type SfrCrfuncW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sfr_crfunc read/write control register"]
    #[inline(always)]
    pub fn sfr_crfunc(&self) -> SfrCrfuncR {
        SfrCrfuncR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - sfr_crfunc read/write control register"]
    #[inline(always)]
    pub fn sfr_crfunc(&mut self) -> SfrCrfuncW<'_, SfrCrfuncSpec> {
        SfrCrfuncW::new(self, 0)
    }
}
#[doc = "See `alu.sv#L136 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_alu/rtl/alu.sv#L136>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_crfunc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_crfunc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCrfuncSpec;
impl crate::RegisterSpec for SfrCrfuncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_crfunc::R`](R) reader structure"]
impl crate::Readable for SfrCrfuncSpec {}
#[doc = "`write(|w| ..)` method takes [`sfr_crfunc::W`](W) writer structure"]
impl crate::Writable for SfrCrfuncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CRFUNC to value 0"]
impl crate::Resettable for SfrCrfuncSpec {}
