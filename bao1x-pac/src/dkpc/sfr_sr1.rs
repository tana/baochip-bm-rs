#[doc = "Register `SFR_SR1` reader"]
pub type R = crate::R<SfrSr1Spec>;
#[doc = "Register `SFR_SR1` writer"]
pub type W = crate::W<SfrSr1Spec>;
#[doc = "Field `sfr_sr1` reader - sfr_sr1 read only status register"]
pub type SfrSr1R = crate::BitReader;
#[doc = "Field `sfr_sr1` writer - sfr_sr1 read only status register"]
pub type SfrSr1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - sfr_sr1 read only status register"]
    #[inline(always)]
    pub fn sfr_sr1(&self) -> SfrSr1R {
        SfrSr1R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sfr_sr1 read only status register"]
    #[inline(always)]
    pub fn sfr_sr1(&mut self) -> SfrSr1W<'_, SfrSr1Spec> {
        SfrSr1W::new(self, 0)
    }
}
#[doc = "See `dkpc.sv#L174 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L174>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_sr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_sr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrSr1Spec;
impl crate::RegisterSpec for SfrSr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_sr1::R`](R) reader structure"]
impl crate::Readable for SfrSr1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_sr1::W`](W) writer structure"]
impl crate::Writable for SfrSr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_SR1 to value 0"]
impl crate::Resettable for SfrSr1Spec {}
