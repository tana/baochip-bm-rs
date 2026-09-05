#[doc = "Register `SFR_OPT3` reader"]
pub type R = crate::R<SfrOpt3Spec>;
#[doc = "Register `SFR_OPT3` writer"]
pub type W = crate::W<SfrOpt3Spec>;
#[doc = "Field `sfr_opt3` reader - sfr_opt3 read/write control register"]
pub type SfrOpt3R = crate::FieldReader;
#[doc = "Field `sfr_opt3` writer - sfr_opt3 read/write control register"]
pub type SfrOpt3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - sfr_opt3 read/write control register"]
    #[inline(always)]
    pub fn sfr_opt3(&self) -> SfrOpt3R {
        SfrOpt3R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - sfr_opt3 read/write control register"]
    #[inline(always)]
    pub fn sfr_opt3(&mut self) -> SfrOpt3W<'_, SfrOpt3Spec> {
        SfrOpt3W::new(self, 0)
    }
}
#[doc = "See `combohasha.sv#L215 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L215>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrOpt3Spec;
impl crate::RegisterSpec for SfrOpt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_opt3::R`](R) reader structure"]
impl crate::Readable for SfrOpt3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_opt3::W`](W) writer structure"]
impl crate::Writable for SfrOpt3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_OPT3 to value 0"]
impl crate::Resettable for SfrOpt3Spec {}
