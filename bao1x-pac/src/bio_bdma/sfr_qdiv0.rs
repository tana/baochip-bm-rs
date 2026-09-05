#[doc = "Register `SFR_QDIV0` reader"]
pub type R = crate::R<SfrQdiv0Spec>;
#[doc = "Register `SFR_QDIV0` writer"]
pub type W = crate::W<SfrQdiv0Spec>;
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
    pub fn unused_div(&mut self) -> UnusedDivW<'_, SfrQdiv0Spec> {
        UnusedDivW::new(self, 0)
    }
    #[doc = "Bit 1 - div_frac read/write control register"]
    #[inline(always)]
    pub fn div_frac(&mut self) -> DivFracW<'_, SfrQdiv0Spec> {
        DivFracW::new(self, 1)
    }
    #[doc = "Bit 2 - div_int read/write control register"]
    #[inline(always)]
    pub fn div_int(&mut self) -> DivIntW<'_, SfrQdiv0Spec> {
        DivIntW::new(self, 2)
    }
}
#[doc = "See `bio_bdma.sv#L511 <https://github.com/baochip/baochip-1x/blob/main/rtl/modul es/bio_bdma/rtl/bio_bdma.sv#L511>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_qdiv0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_qdiv0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrQdiv0Spec;
impl crate::RegisterSpec for SfrQdiv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_qdiv0::R`](R) reader structure"]
impl crate::Readable for SfrQdiv0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_qdiv0::W`](W) writer structure"]
impl crate::Writable for SfrQdiv0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_QDIV0 to value 0"]
impl crate::Resettable for SfrQdiv0Spec {}
