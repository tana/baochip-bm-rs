#[doc = "Register `SFR_RCURST1` reader"]
pub type R = crate::R<SfrRcurst1Spec>;
#[doc = "Register `SFR_RCURST1` writer"]
pub type W = crate::W<SfrRcurst1Spec>;
#[doc = "Field `sfr_rcurst1` reader - sfr_rcurst1 performs action on write of value: 0x55aa"]
pub type SfrRcurst1R = crate::FieldReader<u32>;
#[doc = "Field `sfr_rcurst1` writer - sfr_rcurst1 performs action on write of value: 0x55aa"]
pub type SfrRcurst1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_rcurst1 performs action on write of value: 0x55aa"]
    #[inline(always)]
    pub fn sfr_rcurst1(&self) -> SfrRcurst1R {
        SfrRcurst1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_rcurst1 performs action on write of value: 0x55aa"]
    #[inline(always)]
    pub fn sfr_rcurst1(&mut self) -> SfrRcurst1W<'_, SfrRcurst1Spec> {
        SfrRcurst1W::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L805 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L805>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rcurst1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rcurst1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRcurst1Spec;
impl crate::RegisterSpec for SfrRcurst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rcurst1::R`](R) reader structure"]
impl crate::Readable for SfrRcurst1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rcurst1::W`](W) writer structure"]
impl crate::Writable for SfrRcurst1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RCURST1 to value 0"]
impl crate::Resettable for SfrRcurst1Spec {}
