#[doc = "Register `SFR_INTCR_CRINT4` reader"]
pub type R = crate::R<SfrIntcrCrint4Spec>;
#[doc = "Register `SFR_INTCR_CRINT4` writer"]
pub type W = crate::W<SfrIntcrCrint4Spec>;
#[doc = "Field `crint4` reader - crint read/write control register"]
pub type Crint4R = crate::FieldReader<u16>;
#[doc = "Field `crint4` writer - crint read/write control register"]
pub type Crint4W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - crint read/write control register"]
    #[inline(always)]
    pub fn crint4(&self) -> Crint4R {
        Crint4R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - crint read/write control register"]
    #[inline(always)]
    pub fn crint4(&mut self) -> Crint4W<'_, SfrIntcrCrint4Spec> {
        Crint4W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intcr_crint4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intcr_crint4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIntcrCrint4Spec;
impl crate::RegisterSpec for SfrIntcrCrint4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_intcr_crint4::R`](R) reader structure"]
impl crate::Readable for SfrIntcrCrint4Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_intcr_crint4::W`](W) writer structure"]
impl crate::Writable for SfrIntcrCrint4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_INTCR_CRINT4 to value 0"]
impl crate::Resettable for SfrIntcrCrint4Spec {}
