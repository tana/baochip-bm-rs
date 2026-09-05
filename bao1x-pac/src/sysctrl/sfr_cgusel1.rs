#[doc = "Register `SFR_CGUSEL1` reader"]
pub type R = crate::R<SfrCgusel1Spec>;
#[doc = "Register `SFR_CGUSEL1` writer"]
pub type W = crate::W<SfrCgusel1Spec>;
#[doc = "Field `sfr_cgusel1` reader - sfr_cgusel1 read/write control register"]
pub type SfrCgusel1R = crate::BitReader;
#[doc = "Field `sfr_cgusel1` writer - sfr_cgusel1 read/write control register"]
pub type SfrCgusel1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - sfr_cgusel1 read/write control register"]
    #[inline(always)]
    pub fn sfr_cgusel1(&self) -> SfrCgusel1R {
        SfrCgusel1R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sfr_cgusel1 read/write control register"]
    #[inline(always)]
    pub fn sfr_cgusel1(&mut self) -> SfrCgusel1W<'_, SfrCgusel1Spec> {
        SfrCgusel1W::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L782 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L782>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgusel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgusel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgusel1Spec;
impl crate::RegisterSpec for SfrCgusel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgusel1::R`](R) reader structure"]
impl crate::Readable for SfrCgusel1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgusel1::W`](W) writer structure"]
impl crate::Writable for SfrCgusel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUSEL1 to value 0"]
impl crate::Resettable for SfrCgusel1Spec {}
