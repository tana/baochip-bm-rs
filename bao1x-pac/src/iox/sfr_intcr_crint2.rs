#[doc = "Register `SFR_INTCR_CRINT2` reader"]
pub type R = crate::R<SfrIntcrCrint2Spec>;
#[doc = "Register `SFR_INTCR_CRINT2` writer"]
pub type W = crate::W<SfrIntcrCrint2Spec>;
#[doc = "Field `crint2` reader - crint read/write control register"]
pub type Crint2R = crate::FieldReader<u16>;
#[doc = "Field `crint2` writer - crint read/write control register"]
pub type Crint2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - crint read/write control register"]
    #[inline(always)]
    pub fn crint2(&self) -> Crint2R {
        Crint2R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - crint read/write control register"]
    #[inline(always)]
    pub fn crint2(&mut self) -> Crint2W<'_, SfrIntcrCrint2Spec> {
        Crint2W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intcr_crint2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intcr_crint2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIntcrCrint2Spec;
impl crate::RegisterSpec for SfrIntcrCrint2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_intcr_crint2::R`](R) reader structure"]
impl crate::Readable for SfrIntcrCrint2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_intcr_crint2::W`](W) writer structure"]
impl crate::Writable for SfrIntcrCrint2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_INTCR_CRINT2 to value 0"]
impl crate::Resettable for SfrIntcrCrint2Spec {}
