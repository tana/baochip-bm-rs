#[doc = "Register `SFR_OPT1` reader"]
pub type R = crate::R<SfrOpt1Spec>;
#[doc = "Register `SFR_OPT1` writer"]
pub type W = crate::W<SfrOpt1Spec>;
#[doc = "Field `cr_opt_hashcnt` reader - cr_opt_hashcnt read/write control register"]
pub type CrOptHashcntR = crate::FieldReader<u16>;
#[doc = "Field `cr_opt_hashcnt` writer - cr_opt_hashcnt read/write control register"]
pub type CrOptHashcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - cr_opt_hashcnt read/write control register"]
    #[inline(always)]
    pub fn cr_opt_hashcnt(&self) -> CrOptHashcntR {
        CrOptHashcntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - cr_opt_hashcnt read/write control register"]
    #[inline(always)]
    pub fn cr_opt_hashcnt(&mut self) -> CrOptHashcntW<'_, SfrOpt1Spec> {
        CrOptHashcntW::new(self, 0)
    }
}
#[doc = "See `combohasha.sv#L213 <https://github.com/baochip/baochip-1x/blob/main/rtl/mod ules/crypto_top/rtl/combohasha.sv#L213>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrOpt1Spec;
impl crate::RegisterSpec for SfrOpt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_opt1::R`](R) reader structure"]
impl crate::Readable for SfrOpt1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_opt1::W`](W) writer structure"]
impl crate::Writable for SfrOpt1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_OPT1 to value 0"]
impl crate::Resettable for SfrOpt1Spec {}
