#[doc = "Register `SFR_CGUSEL0` reader"]
pub type R = crate::R<SfrCgusel0Spec>;
#[doc = "Register `SFR_CGUSEL0` writer"]
pub type W = crate::W<SfrCgusel0Spec>;
#[doc = "Field `sfr_cgusel0` reader - sfr_cgusel0 read/write control register"]
pub type SfrCgusel0R = crate::FieldReader;
#[doc = "Field `sfr_cgusel0` writer - sfr_cgusel0 read/write control register"]
pub type SfrCgusel0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - sfr_cgusel0 read/write control register"]
    #[inline(always)]
    pub fn sfr_cgusel0(&self) -> SfrCgusel0R {
        SfrCgusel0R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - sfr_cgusel0 read/write control register"]
    #[inline(always)]
    pub fn sfr_cgusel0(&mut self) -> SfrCgusel0W<'_, SfrCgusel0Spec> {
        SfrCgusel0W::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L774 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L774>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cgusel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cgusel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCgusel0Spec;
impl crate::RegisterSpec for SfrCgusel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cgusel0::R`](R) reader structure"]
impl crate::Readable for SfrCgusel0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cgusel0::W`](W) writer structure"]
impl crate::Writable for SfrCgusel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CGUSEL0 to value 0"]
impl crate::Resettable for SfrCgusel0Spec {}
