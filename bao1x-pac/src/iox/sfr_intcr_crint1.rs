#[doc = "Register `SFR_INTCR_CRINT1` reader"]
pub type R = crate::R<SfrIntcrCrint1Spec>;
#[doc = "Register `SFR_INTCR_CRINT1` writer"]
pub type W = crate::W<SfrIntcrCrint1Spec>;
#[doc = "Field `crint1` reader - crint read/write control register"]
pub type Crint1R = crate::FieldReader<u16>;
#[doc = "Field `crint1` writer - crint read/write control register"]
pub type Crint1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - crint read/write control register"]
    #[inline(always)]
    pub fn crint1(&self) -> Crint1R {
        Crint1R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - crint read/write control register"]
    #[inline(always)]
    pub fn crint1(&mut self) -> Crint1W<'_, SfrIntcrCrint1Spec> {
        Crint1W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L127 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/if sub/rtl/iox.sv#L127>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_intcr_crint1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_intcr_crint1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrIntcrCrint1Spec;
impl crate::RegisterSpec for SfrIntcrCrint1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_intcr_crint1::R`](R) reader structure"]
impl crate::Readable for SfrIntcrCrint1Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_intcr_crint1::W`](W) writer structure"]
impl crate::Writable for SfrIntcrCrint1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_INTCR_CRINT1 to value 0"]
impl crate::Resettable for SfrIntcrCrint1Spec {}
