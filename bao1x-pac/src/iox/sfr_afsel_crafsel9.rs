#[doc = "Register `SFR_AFSEL_CRAFSEL9` reader"]
pub type R = crate::R<SfrAfselCrafsel9Spec>;
#[doc = "Register `SFR_AFSEL_CRAFSEL9` writer"]
pub type W = crate::W<SfrAfselCrafsel9Spec>;
#[doc = "Field `crafsel9` reader - crafsel read/write control register"]
pub type Crafsel9R = crate::FieldReader<u16>;
#[doc = "Field `crafsel9` writer - crafsel read/write control register"]
pub type Crafsel9W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - crafsel read/write control register"]
    #[inline(always)]
    pub fn crafsel9(&self) -> Crafsel9R {
        Crafsel9R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - crafsel read/write control register"]
    #[inline(always)]
    pub fn crafsel9(&mut self) -> Crafsel9W<'_, SfrAfselCrafsel9Spec> {
        Crafsel9W::new(self, 0)
    }
}
#[doc = "See `iox.sv#L73 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/ifs ub/rtl/iox.sv#L73>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_afsel_crafsel9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_afsel_crafsel9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrAfselCrafsel9Spec;
impl crate::RegisterSpec for SfrAfselCrafsel9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_afsel_crafsel9::R`](R) reader structure"]
impl crate::Readable for SfrAfselCrafsel9Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_afsel_crafsel9::W`](W) writer structure"]
impl crate::Writable for SfrAfselCrafsel9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_AFSEL_CRAFSEL9 to value 0"]
impl crate::Resettable for SfrAfselCrafsel9Spec {}
