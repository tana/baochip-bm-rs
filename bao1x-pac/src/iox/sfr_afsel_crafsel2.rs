#[doc = "Register `SFR_AFSEL_CRAFSEL2` reader"]
pub type R = crate::R<SfrAfselCrafsel2Spec>;
#[doc = "Register `SFR_AFSEL_CRAFSEL2` writer"]
pub type W = crate::W<SfrAfselCrafsel2Spec>;
#[doc = "Field `crafsel2` reader - crafsel read/write control register"]
pub type Crafsel2R = crate::FieldReader<u16>;
#[doc = "Field `crafsel2` writer - crafsel read/write control register"]
pub type Crafsel2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - crafsel read/write control register"]
    #[inline(always)]
    pub fn crafsel2(&self) -> Crafsel2R {
        Crafsel2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - crafsel read/write control register"]
    #[inline(always)]
    pub fn crafsel2(&mut self) -> Crafsel2W<'_, SfrAfselCrafsel2Spec> {
        Crafsel2W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrAfselCrafsel2Spec;
impl crate::RegisterSpec for SfrAfselCrafsel2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_afsel_crafsel2::R`](R) reader structure"]
impl crate::Readable for SfrAfselCrafsel2Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_afsel_crafsel2::W`](W) writer structure"]
impl crate::Writable for SfrAfselCrafsel2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_AFSEL_CRAFSEL2 to value 0"]
impl crate::Resettable for SfrAfselCrafsel2Spec {}
