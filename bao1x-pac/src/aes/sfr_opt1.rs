#[doc = "Register `SFR_OPT1` reader"]
pub type R = crate::R<SfrOpt1Spec>;
#[doc = "Register `SFR_OPT1` writer"]
pub type W = crate::W<SfrOpt1Spec>;
#[doc = "Field `sfr_opt1` reader - sfr_opt1 read/write control register"]
pub type SfrOpt1R = crate::FieldReader<u16>;
#[doc = "Field `sfr_opt1` writer - sfr_opt1 read/write control register"]
pub type SfrOpt1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_opt1 read/write control register"]
    #[inline(always)]
    pub fn sfr_opt1(&self) -> SfrOpt1R {
        SfrOpt1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_opt1 read/write control register"]
    #[inline(always)]
    pub fn sfr_opt1(&mut self) -> SfrOpt1W<'_, SfrOpt1Spec> {
        SfrOpt1W::new(self, 0)
    }
}
#[doc = "See `aes.sv#L146 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/cr ypto_top/rtl/aes.sv#L146>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_opt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_opt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
