#[doc = "Register `WTEST` reader"]
pub type R = crate::R<WtestSpec>;
#[doc = "Register `WTEST` writer"]
pub type W = crate::W<WtestSpec>;
#[doc = "Field `wtest` reader - "]
pub type WtestR = crate::FieldReader<u32>;
#[doc = "Field `wtest` writer - "]
pub type WtestW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wtest(&self) -> WtestR {
        WtestR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wtest(&mut self) -> WtestW<'_, WtestSpec> {
        WtestW::new(self, 0)
    }
}
#[doc = "Write test data here\n\nYou can [`read`](crate::Reg::read) this register and get [`wtest::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtest::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WtestSpec;
impl crate::RegisterSpec for WtestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtest::R`](R) reader structure"]
impl crate::Readable for WtestSpec {}
#[doc = "`write(|w| ..)` method takes [`wtest::W`](W) writer structure"]
impl crate::Writable for WtestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WTEST to value 0"]
impl crate::Resettable for WtestSpec {}
