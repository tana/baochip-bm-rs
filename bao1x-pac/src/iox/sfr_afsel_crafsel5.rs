#[doc = "Register `SFR_AFSEL_CRAFSEL5` reader"]
pub type R = crate::R<SfrAfselCrafsel5Spec>;
#[doc = "Register `SFR_AFSEL_CRAFSEL5` writer"]
pub type W = crate::W<SfrAfselCrafsel5Spec>;
#[doc = "Field `crafsel5` reader - crafsel read/write control register"]
pub type Crafsel5R = crate::FieldReader<u16>;
#[doc = "Field `crafsel5` writer - crafsel read/write control register"]
pub type Crafsel5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - crafsel read/write control register"]
    #[inline(always)]
    pub fn crafsel5(&self) -> Crafsel5R {
        Crafsel5R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - crafsel read/write control register"]
    #[inline(always)]
    pub fn crafsel5(&mut self) -> Crafsel5W<'_, SfrAfselCrafsel5Spec> {
        Crafsel5W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrAfselCrafsel5Spec;
impl crate::RegisterSpec for SfrAfselCrafsel5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_afsel_crafsel5::R`](R) reader structure"]
impl crate::Readable for SfrAfselCrafsel5Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_afsel_crafsel5::W`](W) writer structure"]
impl crate::Writable for SfrAfselCrafsel5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_AFSEL_CRAFSEL5 to value 0"]
impl crate::Resettable for SfrAfselCrafsel5Spec {}
