#[doc = "Register `SFR_QDIV3` reader"]
pub type R = crate::R<SfrQdiv3Spec>;
#[doc = "Register `SFR_QDIV3` writer"]
pub type W = crate::W<SfrQdiv3Spec>;
#[doc = "Field `unused_div` reader - unused_div read/write control register"]
pub type UnusedDivR = crate::BitReader;
#[doc = "Field `unused_div` writer - unused_div read/write control register"]
pub type UnusedDivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `div_frac` reader - div_frac read/write control register"]
pub type DivFracR = crate::BitReader;
#[doc = "Field `div_frac` writer - div_frac read/write control register"]
pub type DivFracW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `div_int` reader - div_int read/write control register"]
pub type DivIntR = crate::BitReader;
#[doc = "Field `div_int` writer - div_int read/write control register"]
pub type DivIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - unused_div read/write control register"]
    #[inline(always)]
    pub fn unused_div(&self) -> UnusedDivR {
        UnusedDivR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - div_frac read/write control register"]
    #[inline(always)]
    pub fn div_frac(&self) -> DivFracR {
        DivFracR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - div_int read/write control register"]
    #[inline(always)]
    pub fn div_int(&self) -> DivIntR {
        DivIntR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - unused_div read/write control register"]
    #[inline(always)]
    pub fn unused_div(&mut self) -> UnusedDivW<'_, SfrQdiv3Spec> {
        UnusedDivW::new(self, 0)
    }
    #[doc = "Bit 1 - div_frac read/write control register"]
    #[inline(always)]
    pub fn div_frac(&mut self) -> DivFracW<'_, SfrQdiv3Spec> {
        DivFracW::new(self, 1)
    }
    #[doc = "Bit 2 - div_int read/write control register"]
    #[inline(always)]
    pub fn div_int(&mut self) -> DivIntW<'_, SfrQdiv3Spec> {
        DivIntW::new(self, 2)
    }
}
#[doc = "See `bio_bdma.sv#L514 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L514>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_qdiv3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_qdiv3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrQdiv3Spec;
impl crate::RegisterSpec for SfrQdiv3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_qdiv3::R`](R) reader structure"]
impl crate::Readable for SfrQdiv3Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_qdiv3::W`](W) writer structure"]
impl crate::Writable for SfrQdiv3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_QDIV3 to value 0"]
impl crate::Resettable for SfrQdiv3Spec {}
