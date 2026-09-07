#[doc = "Register `RTEST` reader"]
pub type R = crate::R<RtestSpec>;
#[doc = "Register `RTEST` writer"]
pub type W = crate::W<RtestSpec>;
#[doc = "Field `rtest` reader - "]
pub type RtestR = crate::FieldReader<u32>;
#[doc = "Field `rtest` writer - "]
pub type RtestW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtest(&self) -> RtestR {
        RtestR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtest(&mut self) -> RtestW<'_, RtestSpec> {
        RtestW::new(self, 0)
    }
}
#[doc = "Read test data here\n\nYou can [`read`](crate::Reg::read) this register and get [`rtest::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtest::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtestSpec;
impl crate::RegisterSpec for RtestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtest::R`](R) reader structure"]
impl crate::Readable for RtestSpec {}
#[doc = "`write(|w| ..)` method takes [`rtest::W`](W) writer structure"]
impl crate::Writable for RtestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTEST to value 0"]
impl crate::Resettable for RtestSpec {}
