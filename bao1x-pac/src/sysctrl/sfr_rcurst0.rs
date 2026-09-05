#[doc = "Register `SFR_RCURST0` reader"]
pub type R = crate::R<SfrRcurst0Spec>;
#[doc = "Register `SFR_RCURST0` writer"]
pub type W = crate::W<SfrRcurst0Spec>;
#[doc = "Field `sfr_rcurst0` reader - sfr_rcurst0 performs action on write of value: 0x55aa"]
pub type SfrRcurst0R = crate::FieldReader<u32>;
#[doc = "Field `sfr_rcurst0` writer - sfr_rcurst0 performs action on write of value: 0x55aa"]
pub type SfrRcurst0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sfr_rcurst0 performs action on write of value: 0x55aa"]
    #[inline(always)]
    pub fn sfr_rcurst0(&self) -> SfrRcurst0R {
        SfrRcurst0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sfr_rcurst0 performs action on write of value: 0x55aa"]
    #[inline(always)]
    pub fn sfr_rcurst0(&mut self) -> SfrRcurst0W<'_, SfrRcurst0Spec> {
        SfrRcurst0W::new(self, 0)
    }
}
#[doc = "See `sysctrl.sv#L804 <https://github.com/baochip/baochip-1x/blob/main/rtl/module s/sysctrl/rtl/sysctrl.sv#L804>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_rcurst0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_rcurst0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrRcurst0Spec;
impl crate::RegisterSpec for SfrRcurst0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_rcurst0::R`](R) reader structure"]
impl crate::Readable for SfrRcurst0Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_rcurst0::W`](W) writer structure"]
impl crate::Writable for SfrRcurst0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_RCURST0 to value 0"]
impl crate::Resettable for SfrRcurst0Spec {}
