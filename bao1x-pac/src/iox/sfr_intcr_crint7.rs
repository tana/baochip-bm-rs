#[doc = "Register `SFR_INTCR_CRINT7` reader"]
pub type R = crate::R<SfrIntcrCrint7Spec>;
#[doc = "Register `SFR_INTCR_CRINT7` writer"]
pub type W = crate::W<SfrIntcrCrint7Spec>;
#[doc = "Field `crint7` reader - crint read/write control register"]
pub type Crint7R = crate::FieldReader<u16>;
#[doc = "Field `crint7` writer - crint read/write control register"]
pub type Crint7W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - crint read/write control register"]
    #[inline(always)]
    pub fn crint7(&self) -> Crint7R {
        Crint7R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - crint read/write control register"]
    #[inline(always)]
    pub fn crint7(&mut self) -> Crint7W<'_, SfrIntcrCrint7Spec> {
        Crint7W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intcr_crint7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intcr_crint7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIntcrCrint7Spec;
impl crate::RegisterSpec for SfrIntcrCrint7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_intcr_crint7::R`](R) reader structure"]
impl crate::Readable for SfrIntcrCrint7Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_intcr_crint7::W`](W) writer structure"]
impl crate::Writable for SfrIntcrCrint7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_INTCR_CRINT7 to value 0"]
impl crate::Resettable for SfrIntcrCrint7Spec {}
